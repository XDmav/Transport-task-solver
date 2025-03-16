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
	result_tab: Vec<Vec<isize>>,
	result_sellers_num: usize,
	result_buyers_num: usize,
	price: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
	        sellers_num: 3,
	        buyers_num: 3,
	        prod_tab: vec![vec![8, 6, 5], vec![1, 4, 4], vec![8, 2, 3]],
	        result_tab: Vec::new(),
	        sellers_need: vec![30, 80, 40],
	        buyers_need: vec![50, 60, 10],
	        result_sellers_num: 3,
	        result_buyers_num: 3,
	        price: 0,
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
			    prod_tab.resize(self.result_sellers_num, vec![0; self.result_buyers_num]);
		    }
		    Ordering::Greater => {
			    self.result_buyers_num += 1;
			    buyers_need.resize(self.result_buyers_num, seller_need_sum - buyers_need_sum);
			    for row in prod_tab.iter_mut() {
				    row.resize(self.result_buyers_num, 0)
			    }
		    }
		    Ordering::Equal => {}
	    }
	    
	    self.result_tab = vec![vec![-1; self.result_buyers_num]; self.result_sellers_num];
	    
	    let mut seller_id = 0;
	    let mut buyer_id = 0;
	    let mut sellers_need_clone = sellers_need.clone();
	    let mut buyers_need_clone = buyers_need.clone();
	    while seller_id < self.result_sellers_num && buyer_id < self.result_buyers_num {
		    match sellers_need_clone[seller_id].cmp(&buyers_need_clone[buyer_id]) {
			    Ordering::Less => {
				    self.result_tab[seller_id][buyer_id] = sellers_need_clone[seller_id] as isize;
				    buyers_need_clone[buyer_id] -= sellers_need_clone[seller_id];
				    sellers_need_clone[seller_id] = 0;
				    seller_id += 1;
			    }
			    Ordering::Equal => {
				    self.result_tab[seller_id][buyer_id] = sellers_need_clone[seller_id] as isize;
				    buyers_need_clone[buyer_id] = 0;
				    sellers_need_clone[seller_id] = 0;
				    seller_id += 1;
			    }
			    Ordering::Greater => {
				    self.result_tab[seller_id][buyer_id] = buyers_need_clone[buyer_id] as isize;
				    sellers_need_clone[seller_id] -= buyers_need_clone[buyer_id];
				    buyers_need_clone[buyer_id] = 0;
				    buyer_id += 1;
			    }
		    }
	    }
	    
	    loop {
		    match self.is_valid(&prod_tab) {
			    None => break,
			    Some((i, j)) => {
				    let mut path = vec![(i, j)];
				    self.move_next(&mut path, i, j, Direction::None, i, j);
				    
				    let mut min = isize::MAX;
				    for (cur_i, cur_j) in path.iter().skip(1).step_by(2) {
					    if min > self.result_tab[*cur_i][*cur_j] {
						    min = self.result_tab[*cur_i][*cur_j];
					    }
				    }
				    
				    self.result_tab[i][j] = min;
				    
				    let mut first = true;
				    for (index, (cur_i, cur_j)) in path.iter().skip(1).enumerate() {
					    if index % 2 == 0 {
						    self.result_tab[*cur_i][*cur_j] -= min;
						    
						    if first && self.result_tab[*cur_i][*cur_j] == 0 {
							    self.result_tab[*cur_i][*cur_j] = -1;
							    first = false;
						    }
					    } else {
						    self.result_tab[*cur_i][*cur_j] += min;
					    }
				    }
			    }
		    }
	    }
	    
	    self.price = 0;
	    for (i, item_i) in self.result_tab.iter().enumerate() {
		    for (j, item_j) in item_i.iter().enumerate() {
			    if *item_j > 0 {
				    self.price += *item_j as usize * prod_tab[i][j]
			    }
		    }
	    }
    }
	
	fn move_next(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, except: Direction, origin_i: usize, origin_j: usize) -> bool {
		match except {
			Direction::Vertical => {
				if self.move_left(path, i, j, origin_i, origin_j) ||
					self.move_right(path, i, j, origin_i, origin_j) {
					return true;
				}
			}
			Direction::Horizontal => {
				if self.move_up(path, i, j, origin_i, origin_j) ||
					self.move_down(path, i, j, origin_i, origin_j) {
					return true;
				}
			}
			Direction::None => {
				if self.move_right(path, i, j, origin_i, origin_j) ||
					self.move_left(path, i, j, origin_i, origin_j) ||
					self.move_up(path, i, j, origin_i, origin_j) ||
					self.move_down(path, i, j, origin_i, origin_j) {
					return true;
				}
			}
		};
		
		false
	}
	
	fn move_right(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, origin_i: usize, origin_j: usize) -> bool {
		if j >= self.buyers_num {
			return false
		}
		
		for q in (j + 1)..self.result_buyers_num {
			if self.move_path(path, i, q, Direction::Horizontal, origin_i, origin_j) {
				return true
			};
		}
		
		false
	}
	
	fn move_left(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, origin_i: usize, origin_j: usize) -> bool {
		if j == 0 {
			return false
		}
		
		for q in 0..j {
			if self.move_path(path, i, q, Direction::Horizontal, origin_i, origin_j) {
				return true
			};
		}
		
		false
	}
	
	fn move_up(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, origin_i: usize, origin_j: usize) -> bool {
		if i == 0 {
			return false
		}
		
		for p in 0..i {
			if self.move_path(path, p, j, Direction::Vertical, origin_i, origin_j) {
				return true
			};
		}
		
		false
	}
	
	fn move_down(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, origin_i: usize, origin_j: usize) -> bool {
		if i >= self.sellers_num {
			return false
		}
		
		for p in (i + 1)..self.result_sellers_num {
			if self.move_path(path, p, j, Direction::Vertical, origin_i, origin_j) {
				return true
			};
		}
		
		false
	}
	
	fn move_path(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, except: Direction, origin_i: usize, origin_j: usize) -> bool {
		if origin_i == i && origin_j == j {
			return true;
		}
		
		if self.result_tab[i][j] != -1 {
			if path.iter().any(|(old_i, old_j)| *old_i == i && *old_j == j) {
				return false;
			}
			
			path.push((i, j));
			
			if self.move_next(path, i, j, except, origin_i, origin_j) {
				return true;
			};
			
			path.pop();
		}
		
		false
	}
	
	fn check_next(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, except: Direction, sellers_potential: &mut Vec<Option<isize>>, buyers_potential: &mut Vec<Option<isize>>, prod_tab: &Vec<Vec<usize>>) {
		match except {
			Direction::Vertical => {
				self.check_left(path, i, j, sellers_potential, buyers_potential, prod_tab);
				self.check_right(path, i, j, sellers_potential, buyers_potential, prod_tab);
			}
			Direction::Horizontal => {
				self.check_up(path, i, j, sellers_potential, buyers_potential, prod_tab);
				self.check_down(path, i, j, sellers_potential, buyers_potential, prod_tab);
			}
			Direction::None => {
				self.check_right(path, i, j, sellers_potential, buyers_potential, prod_tab);
				self.check_left(path, i, j, sellers_potential, buyers_potential, prod_tab);
				self.check_up(path, i, j, sellers_potential, buyers_potential, prod_tab);
				self.check_down(path, i, j, sellers_potential, buyers_potential, prod_tab);
			}
		};
	}
	
	fn check_right(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, sellers_potential: &mut Vec<Option<isize>>, buyers_potential: &mut Vec<Option<isize>>, prod_tab: &Vec<Vec<usize>>) {
		if j >= self.buyers_num {
			return;
		}
		
		for q in (j + 1)..self.result_buyers_num {
			self.check_path(path, i, q, Direction::Horizontal, sellers_potential, buyers_potential, prod_tab)
		}
	}
	
	fn check_left(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, sellers_potential: &mut Vec<Option<isize>>, buyers_potential: &mut Vec<Option<isize>>, prod_tab: &Vec<Vec<usize>>) {
		if j == 0 {
			return;
		}
		
		for q in 0..j {
			self.check_path(path, i, q, Direction::Horizontal, sellers_potential, buyers_potential, prod_tab)
		}
	}
	
	fn check_up(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, sellers_potential: &mut Vec<Option<isize>>, buyers_potential: &mut Vec<Option<isize>>, prod_tab: &Vec<Vec<usize>>) {
		if i == 0 {
			return;
		}
		
		for p in 0..i {
			self.check_path(path, p, j, Direction::Vertical, sellers_potential, buyers_potential, prod_tab)
		}
	}
	
	fn check_down(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, sellers_potential: &mut Vec<Option<isize>>, buyers_potential: &mut Vec<Option<isize>>, prod_tab: &Vec<Vec<usize>>) {
		if i >= self.sellers_num {
			return;
		}
		
		for p in (i + 1)..self.result_sellers_num {
			self.check_path(path, p, j, Direction::Vertical, sellers_potential, buyers_potential, prod_tab)
		}
	}
	
	fn check_path(&self, path: &mut Vec<(usize, usize)>, i: usize, j: usize, except: Direction, sellers_potential: &mut Vec<Option<isize>>, buyers_potential: &mut Vec<Option<isize>>, prod_tab: &Vec<Vec<usize>>) {
		if self.result_tab[i][j] != -1 {
			if let Some(index) = path.iter().position(|(old_i, old_j)| *old_i == i && *old_j == j) {
				path.remove(index);
				
				if let Some(seller_potential) = sellers_potential[i] {
					buyers_potential[j] = Some(prod_tab[i][j] as isize - seller_potential);
				} else if let Some(buyer_potential) = buyers_potential[i] {
					sellers_potential[i] = Some(prod_tab[i][j] as isize - buyer_potential);
				}
				
				self.check_next(path, i, j, except, sellers_potential, buyers_potential, prod_tab);
			}
		}
	}
	
	fn is_valid(&self, prod_tab: &Vec<Vec<usize>>) -> Option<(usize, usize)> {
		let mut sellers_potential: Vec<Option<isize>> = vec![None; self.result_sellers_num];
		let mut buyers_potential: Vec<Option<isize>> = vec![None; self.result_buyers_num];
		
		let mut path = Vec::new();
		for (i, item_i) in self.result_tab.iter().enumerate() {
			for (j, item_j) in item_i.iter().enumerate() {
				if *item_j != -1 {
					path.push((i, j));
				}
			}
		}
		
		let (first_i, first_j) = path.last().unwrap();
		let (first_i, first_j) = (*first_i, *first_j);
		
		sellers_potential[first_i] = Some(0);
		buyers_potential[first_j] = Some(prod_tab[first_i][first_j] as isize);
		
		path.pop();
		self.check_next(&mut path, first_i, first_j, Direction::None, &mut sellers_potential, &mut buyers_potential, prod_tab);
		
		let mut min = 0;
		let mut result = None;
		for (i, item_i) in self.result_tab.iter().enumerate() {
			for (j, item_j) in item_i.iter().enumerate() {
				if *item_j == -1 {
					let num = prod_tab[i][j] as isize - (sellers_potential[i].unwrap() + buyers_potential[j].unwrap());
					if num < min {
						min = num;
						result = Some((i, j));
					}
				}
			}
		}
		
		result
	}
}

#[derive(Debug)]
enum Direction {
	Vertical,
	Horizontal,
	None,
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
				        row.col(|ui| {
					        ui.label("Потребность");
				        });
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
	        
	        ui.label("Цена: ".to_owned() + self.price.to_string().as_ref());
	        
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
						            if self.result_tab[i][j] == -1 {
							            ui.label("0");
						            } else {
							            ui.label(self.result_tab[i][j].to_string());
						            }
					            });
				            };
			            });
		            }
	            });
        });
    }
}