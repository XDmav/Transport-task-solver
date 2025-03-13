#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::Ordering;
use eframe::egui::{CentralPanel, Color32, DragValue, FontFamily, FontId, IconData, TextStyle, Ui, ViewportBuilder, Visuals};
use eframe::{egui, NativeOptions};
use egui_extras::{Column, TableBuilder, TableRow};

fn main() -> eframe::Result {
    env_logger::init();
    
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_icon(IconData::default())
            .with_inner_size([600.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Lab6",
        options,
        Box::new(|cc| {
            if cc.egui_ctx.style().visuals.dark_mode {
                cc.egui_ctx.set_visuals(Visuals {
                    override_text_color: Some(Color32::from_rgb(255, 255, 255)),
                    ..cc.egui_ctx.style().visuals.clone()
                });
            }
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles.insert(
                TextStyle::Body,
                FontId::new(16.0, FontFamily::Proportional),
            );
            cc.egui_ctx.set_style(style);
            
            let mut result = Box::<MyApp>::default();
            result.change();
            Ok(result)
        }),
    )
}

struct MyApp {
    sellers_num: usize,
	buyers_num: usize,
	sellers_need: Vec<usize>,
	buyers_need: Vec<usize>,
	prod_tab: Vec<Vec<usize>>,
	result_tab: Vec<Vec<usize>>,
	result_sellers_num: usize,
	result_buyers_num: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
	        sellers_num: 3,
	        buyers_num: 3,
	        prod_tab: vec![vec![1; 3]; 3],
	        result_tab: Vec::new(),
	        sellers_need: vec![10; 3],
	        buyers_need: vec![10; 3],
	        result_sellers_num: 3,
	        result_buyers_num: 3,
        }
    }
}

fn add_drag_value(ui: &mut Ui, value: &mut usize, changed: &mut bool) {
    if ui.add_sized([60.0, 15.0], DragValue::new(value).range(1..=usize::MAX)).changed() {
        *changed = true
    };
}

impl MyApp {
    fn change(&mut self) {
        let mut prod_tab = self.prod_tab.clone();
	    
	    let mut sellers_need = self.sellers_need.clone();
	    let mut buyers_need = self.buyers_need.clone();
	    let seller_need_sum: usize = self.sellers_need.iter().sum();
	    let buyers_need_sum: usize = self.buyers_need.iter().sum();
	    self.result_sellers_num = self.sellers_num;
	    self.result_buyers_num = self.buyers_num;
	    
	    match seller_need_sum.cmp(&buyers_need_sum) {
		    Ordering::Less => {
			    self.result_sellers_num += 1;
			    sellers_need.resize(self.result_sellers_num, buyers_need_sum - seller_need_sum);
			    prod_tab.resize(self.result_sellers_num, Vec::new());
		    }
		    Ordering::Greater => {
			    self.result_buyers_num += 1;
			    buyers_need.resize(self.result_buyers_num, seller_need_sum - buyers_need_sum);
			    for row in prod_tab.iter_mut() {
				    row.resize(self.result_buyers_num, 1)
			    }
		    }
		    Ordering::Equal => {}
	    }
	    
	    self.result_tab = vec![vec![0; self.result_buyers_num]; self.result_sellers_num];
	    
	    let mut seller_id = 0;
	    let mut buyer_id = 0;
	    let mut sellers_need_clone = sellers_need.clone();
	    let mut buyers_need_clone = buyers_need.clone();
	    while seller_id < self.result_sellers_num || buyer_id < self.result_buyers_num {
		    match sellers_need_clone[seller_id].cmp(&buyers_need_clone[buyer_id]) {
			    Ordering::Less => {
				    self.result_tab[seller_id][buyer_id] = sellers_need_clone[seller_id];
				    buyers_need_clone[buyer_id] -= sellers_need_clone[seller_id];
				    sellers_need_clone[seller_id] = 0;
				    seller_id += 1;
			    }
			    Ordering::Equal => {
				    self.result_tab[seller_id][buyer_id] = sellers_need_clone[seller_id];
				    buyers_need_clone[buyer_id] = 0;
				    sellers_need_clone[seller_id] = 0;
				    buyer_id += 1;
				    seller_id += 1;
			    }
			    Ordering::Greater => {
				    self.result_tab[seller_id][buyer_id] = buyers_need_clone[buyer_id];
				    sellers_need_clone[seller_id] -= buyers_need_clone[buyer_id];
				    buyers_need_clone[buyer_id] = 0;
				    buyer_id += 1;
			    }
		    }
	    }
	    
	    
    }
}

fn get_header(row: &mut TableRow, buyers_num: usize) {
	row.col(|_ui| {});
	for j in 0..buyers_num {
		row.col(|ui| {
			ui.label("B".to_owned() + (j + 1).to_string().as_ref());
		});
	}
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
	        let mut changed = false;
         
	        ui.horizontal(|ui| {
		        ui.label("Кол-во продавцов: ");
		        add_drag_value(ui, &mut self.sellers_num, &mut changed);
		        ui.label("Кол-во покупателей: ");
		        add_drag_value(ui, &mut self.buyers_num, &mut changed);
	        });
	        
	        ui.add_space(10.0);
	        
	        if changed {
		        self.prod_tab.resize(self.sellers_num, Vec::new());
		        self.sellers_need.resize(self.sellers_num, 10);
		        self.buyers_need.resize(self.buyers_num, 10);
		        for row in self.prod_tab.iter_mut() {
			        row.resize(self.buyers_num, 1)
		        }
	        }
	        
	        TableBuilder::new(ui)
		        .id_salt(1)
		        .striped(true)
		        .columns(Column::auto().at_least(35.0), self.buyers_num + 2)
		        .header(20.0, |mut row| {
			        get_header(&mut row, self.buyers_num);
			        row.col(|ui| {
				        ui.label("Запас");
			        });
		        })
		        .body(|mut body| {
			        for i in 0..self.sellers_num {
				        body.row(20.0, |mut row| {
					        row.col(|ui| {
						        ui.label("A".to_owned() + (i + 1).to_string().as_ref());
					        });
					        for j in 0..self.buyers_num {
						        row.col(|ui| {
							        add_drag_value(ui, &mut self.prod_tab[i][j], &mut changed);
						        });
					        }
					        row.col(|ui| {
						        add_drag_value(ui, &mut self.sellers_need[i], &mut changed);
					        });
				        });
			        }
			        body.row(20.0, |mut row| {
				        row.col(|_ui| {});
				        for j in 0..self.buyers_num {
					        row.col(|ui| {
						        add_drag_value(ui, &mut self.buyers_need[j], &mut changed);
					        });
				        }
			        });
		        });
	        
	        ui.add_space(40.0);
            
            if changed {
                self.change()
            }
            
            TableBuilder::new(ui)
	            .id_salt(2)
	            .striped(true)
	            .columns(Column::auto().at_least(35.0), self.result_buyers_num + 1)
	            .header(20.0, |mut row| {
		            get_header(&mut row, self.result_buyers_num);
	            })
	            .body(|mut body| {
		            for i in 0..self.result_sellers_num {
			            body.row(20.0, |mut row| {
				            row.col(|ui| {
					            ui.label("A".to_owned() + (i + 1).to_string().as_ref());
				            });
				            for j in 0..self.result_buyers_num {
					            row.col(|ui| {
						            ui.label(self.result_tab[i][j].to_string());
					            });
				            }
			            });
		            }
	            });
        });
    }
}