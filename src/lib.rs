mod utils;


use las::{Read, Reader};
use las::{Builder,Write, Writer, Point,Header};

use las::Color;

use las::point::Format;

use std::f64::consts::PI;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use std::io::Cursor;

use wasm_bindgen::JsCast;

use web_sys::console::log_1;

use std::{env, io};
use once_cell::sync::OnceCell;

static  lasdata: OnceCell<Mutex<Vec<u8>>> =OnceCell::new();
static  tmplasdata: OnceCell<Mutex<Vec<u8>>> =OnceCell::new();
static  compute_func: OnceCell<Mutex<Vec< ( f64 , fn( f64 ,Point) -> Point )  >>> =OnceCell::new();

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = set_las_data)]
pub fn set_las_data ( d: Vec<u8> ) {
    let u = lasdata.get_or_init(|| Mutex::new(Vec::new()));
    u.lock().unwrap().truncate(0);
    u.lock().unwrap().extend(d);
} 

#[wasm_bindgen(js_name = get_las_data)]
pub fn get_las_data ( )-> Vec<u8> {

    let data =  lasdata.get().unwrap() ; 
    return  data.lock().unwrap().clone() ;
} 

#[wasm_bindgen(js_name = get_tmplas_data)]
pub fn get_tmplas_data ( )-> Vec<u8> {

    let data =  tmplasdata.get().unwrap() ;
    
    return  data.lock().unwrap().clone() ;
} 


fn set_func ( tp : ( f64 , fn( f64 ,Point)-> Point  ))  {
    let u = compute_func.get_or_init(|| Mutex::new(Vec::new()));
    u.lock().unwrap().push(  tp  ) ;  
}


#[wasm_bindgen(js_name = rot_x)]
pub fn rot_x ( d : f64 ){

    let rad = d/180. * PI;
    set_func (  ( rad ,  | rad : f64, mut p: Point| -> Point { 
            let y = p.y ; let z = p.z;
            p.y =  y * rad.cos()   +  z * rad.sin();  
            p.z =  y * ( - rad.sin() ) + z * rad.cos();  
            return p;  
        } ));    
}

#[wasm_bindgen(js_name = rot_y)]
pub fn rot_y ( d : f64 ){

    let rad = d/180. * PI;
    set_func (  ( rad ,  | rad : f64, mut p: Point| -> Point { 
            let x = p.x ; let z = p.z;
            p.x =  x * rad.cos()   +  z * ( - rad.sin());  
            p.z =  x * rad.sin() +  z * rad.cos();  
            return p;  
        } ));    
}

#[wasm_bindgen(js_name = rot_z)]
pub fn rot_z ( d : f64 ){

    let rad = d/180. * PI;
    set_func (  ( rad ,  | rad : f64, mut p: Point| -> Point { 
            let x = p.x ; let y = p.y;
            p.x =  x * rad.cos()   +  y * rad.sin();  
            p.y =  x * ( -1.0 * rad.sin() ) + y * rad.cos();  
            return p;  
        } ));    
}

#[wasm_bindgen(js_name = add_x)]
pub fn add_x ( num : f64 ) {00
 
    set_func (  ( num ,  | num : f64, mut p: Point| -> Point {  
        
    /*    log_1(&JsValue::from( num.to_string() ));            
      */
        // log_1(&JsValue::from( p.x.to_string() ));
      
        p.x +=  num  ; return p; 
    
    } )  );    
}

#[wasm_bindgen(js_name = add_y)]
pub fn add_y ( num : f64 ) {

    set_func ( ( num ,  | num : f64, mut p: Point| -> Point {  p.y +=  num  ; return p; } ));    
}

#[wasm_bindgen(js_name = add_z)]
pub fn add_z ( num : f64 ) {

    set_func (( num ,  | num : f64, mut p: Point| -> Point {  p.z +=  num  ; return p; } ));    
}

fn create_point ( p : Point ) -> Point {
        let mut point = Point::default(); // default points don't have any optional attributes
        point.x= p.x;
        point.y= p.y;
        point.z= p.z;
    	point.gps_time = Some(0.); // point format 1 requires gps time
    	
	    if let Some(color) = p.color {
        	point.color =  Some(Color::new(
        	color.red,
            color.green,
            color.blue
        	));
    	}
    	return point ;
}



#[wasm_bindgen(js_name = get_points)]
pub fn get_points( )  -> Vec<f64>   {

  let data = get_las_data();
  let mut file = Cursor::new(data);   
  let mut reader = Reader::new(file).unwrap();
  let points = reader.points();
  let mut result: Vec<f64> = vec!{};
  
   let mut i = 0;
   for mut point in points {
         let p =point. unwrap(); 
          result.push(p.x);
          result.push(p.y);
          result.push(p.z);
    }
    return result;
}

#[wasm_bindgen(js_name = add_las_data)]
pub fn add_las_data ( d: Vec<u8> ) {
    
   let tmpdata = tmplasdata.get_or_init(|| Mutex::new(Vec::new()));
   tmpdata.lock().unwrap().truncate(0);
   tmpdata.lock().unwrap().extend(d);
   
   let mut tmpfile = Cursor::new(get_tmplas_data());   
   let mut tmpreader = Reader::new(tmpfile).unwrap();
   
   let mut builder = Builder::default();
   builder.point_format = Format::new(3).unwrap();
   let mut writer = Writer::new(Cursor::new(Vec::new()), builder.into_header().unwrap()).unwrap();
 
   for wrapped_point_tmp in tmpreader.points() {
    	// log_1(&JsValue::from( "test" )); 
    	  let mut point_tmp  = create_point(wrapped_point_tmp.unwrap() );
    	  writer.write(point_tmp.clone()).unwrap();
    }	

   tmpdata.lock().unwrap().truncate(0);
   
   let mut file = Cursor::new(get_las_data()); 
   let mut reader = Reader::new(file).unwrap();
  
   for wrapped_point in reader.points() {
    	  // log_1(&JsValue::from( "test" )); 
        let mut point  = create_point(wrapped_point.unwrap() );
        writer.write(point.clone()).unwrap();
    }	

  
    let cursor = writer.into_inner().unwrap();
    	
	set_las_data( cursor.into_inner() );     
  
    // u.lock().unwrap().truncate(0);
} 

#[wasm_bindgen(js_name = computation)]
pub fn computation( )  {

   let data = get_las_data();
   let mut file = Cursor::new(data);   
   let mut reader = Reader::new(file).unwrap();
   
   let mut builder = Builder::default();
   builder.point_format = Format::new(3).unwrap();
   let mut writer = Writer::new(Cursor::new(Vec::new()), builder.into_header().unwrap()).unwrap();
   
   let u = compute_func.get_or_init(|| Mutex::new(Vec::new()));
        
   for wrapped_point in reader.points() {
    	// log_1(&JsValue::from( "test" )); 
    	let mut point  = create_point(wrapped_point.unwrap() );
          
          for ( num , f ) in u.lock().unwrap().iter() {
           
                point = f ( *num , point ) ;
           }
    	  // assert!(writer.write(point).is_err()); 
    	   writer.write(point.clone()).unwrap();
    }	
    	
	writer.close().unwrap();
    	
    let cursor = writer.into_inner().unwrap();
    	
	set_las_data( cursor.into_inner() ); 
    	
	 u.lock().unwrap().truncate(0);
}