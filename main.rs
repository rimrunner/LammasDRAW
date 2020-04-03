/*
NOTE: English comments are "real comments", Finnish comments are programmer's personal notes to be removed

compile: cargo run --features unsafe_textures
run: <exefilepath> <ttf-font file path>
*/

extern crate sdl2;

#[cfg(feature = "unsafe_textures")]
use std::env;
#[cfg(feature = "unsafe_textures")]
use std::path::Path;
#[cfg(feature = "unsafe_textures")]
use sdl2::event::Event;
#[cfg(feature = "unsafe_textures")]
use sdl2::keyboard::Keycode;
//use sdl2::image::{LoadTexture, InitFlag};
#[cfg(feature = "unsafe_textures")]
use sdl2::rect::Rect;
//use sdl2::render::TextureQuery;
#[cfg(feature = "unsafe_textures")]
use sdl2::pixels::Color;
#[cfg(feature = "unsafe_textures")]
use sdl2::gfx::primitives::DrawRenderer;
#[cfg(feature = "unsafe_textures")]
use sdl2_sys::SDL_DestroyTexture;
#[cfg(feature = "unsafe_textures")]
use std::collections::HashMap;



#[cfg(feature = "unsafe_textures")]
fn render_screen(mut sdl_master: &mut SDLMasterVars, mut gridvec_obj: &mut Gridvec, mut gridvars: &mut GridMasterVars, mut iface: &mut Interface) -> Result<(), String> {

    sdl_master.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    sdl_master.canvas.clear();

    let tempwin = sdl_master.canvas.window_mut();
    let size = tempwin.size();
    let winwidth = size.0;
    let winheight = size.1;

    //Cursor colors
    let curcol1 = Color::RGBA(100, 100, 100, 255); //Gray
    let curcol2 = Color::RGBA(255, 0, 0, 255); //Red

    if iface.program_mode == 1 || iface.program_mode == 2 { //1 = Normal drawing mode, 2 = ANSI char select screen
    
        //Pitäisikö tämä siirtää interfaceen?
        //        let panel = Rect::new(0 as i32, (winheight-100) as i32, (gridvars.char_w*(gridvars.grid_x as i16)) as u32, winheight as u32);
        let panel = Rect::new(0 as i32, (winheight-50) as i32, winwidth as u32, 50); //viimeinen oli winheight
        sdl_master.canvas.set_draw_color(Color::RGBA(130, 130, 130, 255));
        sdl_master.canvas.fill_rect(panel)?;

        //Rendering palette
        let mut palrect = Rect::new((winwidth-16) as i32, (winheight-46) as i32, 16, 16);  //Paneeli itse on 50, ja siitä pois selektorin korkeus
        if let Some(color) = sdl_master.colortable.get(&16) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&15) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&14) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&13) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&12) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&11) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&10) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&9) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&8) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&7) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&6) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&5) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&4) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&3) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&2) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;
        palrect.x = palrect.x-16;
        if let Some(color) = sdl_master.colortable.get(&1) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(palrect)?;

        //Rendering color and char boxes to the panel
        //-Merkit väreineen ja taustaväreineen
        //Ensin rendataan ne taustat

        let charbox1 = Rect::new(10, (winheight - 46) as i32, 16, 16);
        let charbox2 = Rect::new(30, (winheight - 46) as i32, 16, 16);
        let charbox3 = Rect::new(50, (winheight - 46) as i32, 16, 16);
        let charbox4 = Rect::new(70, (winheight - 46) as i32, 16, 16);
        let charbox5 = Rect::new(90, (winheight - 46) as i32, 16, 16);
        
        if let Some(color) = sdl_master.colortable.get(&iface.bcolor1) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(charbox1)?;
        if let Some(color) = sdl_master.colortable.get(&iface.bcolor2) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(charbox2)?;
        if let Some(color) = sdl_master.colortable.get(&iface.bcolor3) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(charbox3)?;
        if let Some(color) = sdl_master.colortable.get(&iface.bcolor4) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(charbox4)?;
        if let Some(color) = sdl_master.colortable.get(&iface.bcolor5) {sdl_master.canvas.set_draw_color(color.clone());}
        sdl_master.canvas.fill_rect(charbox5)?;

        if let Some(pb1) = iface.panelbox1.as_ref() {sdl_master.canvas.copy(pb1, None, charbox1)?;}
        if let Some(pb2) = iface.panelbox2.as_ref() {sdl_master.canvas.copy(pb2, None, charbox2)?;}
        if let Some(pb3) = iface.panelbox3.as_ref() {sdl_master.canvas.copy(pb3, None, charbox3)?;}
        if let Some(pb4) = iface.panelbox4.as_ref() {sdl_master.canvas.copy(pb4, None, charbox4)?;}
        if let Some(pb5) = iface.panelbox5.as_ref() {sdl_master.canvas.copy(pb5, None, charbox5)?;}

	//Tämän voisi optimoida tästä pysyväksi tekstuuriksi, mutta sitten pitäisi ehkä tehdä joku init_interface()
        let drawmode_surface = sdl_master.font.render("|ALL | CHAR+COL | CHAR | BG | CHARCOL|").blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
        let drawmode_text = sdl_master.texture_creator.create_texture_from_surface(drawmode_surface).map_err(|e| e.to_string())? ;
        let dmrect = Rect::new(120, (winheight-50) as i32, 191, 17);
        sdl_master.canvas.copy(&drawmode_text, None, dmrect)?;
	unsafe { SDL_DestroyTexture((&drawmode_text as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture); }

        //Drawmode pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        //        let mut dmselrect = Rect::new(126, ((gridvars.char_w*(gridvars.grid_y as i16)+14 as i16)) as i32, 10, 4);
        let mut dmselrect = Rect::new(126, (winheight-33) as i32, 10, 4);
        if iface.dm_selector == 1 {dmselrect.x = dmselrect.x + 5;}
        else if iface.dm_selector == 2 {dmselrect.x = dmselrect.x + 40;}
        else if iface.dm_selector == 3 {dmselrect.x = dmselrect.x + 80;}
        else if iface.dm_selector == 4 {dmselrect.x = dmselrect.x + 120;}
        else if iface.dm_selector == 5 {dmselrect.x = dmselrect.x + 160;}
        sdl_master.canvas.fill_rect(dmselrect)?;

        //Character color pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        let mut ccselrect = Rect::new((winwidth-272) as i32, (winheight-50) as i32, 16, 4);
        ccselrect.x = ccselrect.x + (16*iface.cc_selector as i32);
        sdl_master.canvas.fill_rect(ccselrect)?;

        //Background color pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        let mut bcselrect = Rect::new((winwidth-272) as i32, (winheight-30) as i32, 16, 4);
        bcselrect.x = bcselrect.x + (16*iface.bc_selector as i32);
        sdl_master.canvas.fill_rect(bcselrect)?;

        //Charbox pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        let mut cbselrect = Rect::new(10, (winheight-30) as i32, 16, 4);
        cbselrect.x = cbselrect.x + (20*(iface.box_selector-1) as i32);
        sdl_master.canvas.fill_rect(cbselrect)?;

        //Cursor coordinates on panel
        let mut costr = format!("{}, {}", iface.cursor_x, iface.cursor_y).to_string();
        if iface.cursor_x < 10 { costr = format!(" {}", costr); }  //Add space for small values in order to prevent comma from moving
        let ref costr = &costr;
//        let coord_rect = Rect::new((winwidth-(costr.len() as u32)) as i32, (winheight-10 as i32), (costr.len()*12) as u32, 16);
        let coord_rect = Rect::new((winwidth-100) as i32, (winheight-15) as i32, (costr.len()*16) as u32, 16);
        let coordsurface = sdl_master.font.render(costr as &str).blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
        let coordtexture = sdl_master.texture_creator.create_texture_from_surface(coordsurface).map_err(|e| e.to_string())?;
        sdl_master.canvas.copy(&coordtexture, None, coord_rect)?;
	unsafe { SDL_DestroyTexture((&coordtexture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture); }
        
        draw_grid(&mut gridvars, &mut gridvec_obj, &mut sdl_master)?;
        //change_gridunit_texture(&mut gridvec_obj, &mut sdl_master, 2, 2, Color::RGBA(255, 0, 0, 255));

        if iface.program_mode != 2 { //Drawing cursor is not rendered when selecting a character

        //Rendering the cursor
        //Upperline
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, (iface.cursor_x+1)*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, curcol1)?;
        //Left line
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, iface.cursor_x*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, curcol1)?;
        //Right line
        sdl_master.canvas.line((iface.cursor_x+1)*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, (iface.cursor_x+1)*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, curcol1)?;
        //Bottomline
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, (iface.cursor_x+1)*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, curcol1)?;

        //Inner cursor rectangle
        //Upperline
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w+2 as i16, iface.cursor_y*gridvars.char_h+1 as i16, (iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y*gridvars.char_h)+1 as i16, curcol2)?;
        //Left line
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w+1 as i16, iface.cursor_y*gridvars.char_h+1 as i16, iface.cursor_x*gridvars.char_w+1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, curcol2)?;
        //Right line
        sdl_master.canvas.line((iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y*gridvars.char_h)+1 as i16, (iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, curcol2)?;
        //Bottomline
            sdl_master.canvas.line(iface.cursor_x*gridvars.char_w+1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, (iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, curcol2)?;

        }

        if iface.program_mode == 2 {  //2 = Character selection screen
            //Character selection screen
            let x_point = (((winwidth as i16) - iface.charscreenx*gridvars.char_w+1) / 2) as i32;
            let y_point = 80i32;
            let selectbgrect = Rect::new(x_point, y_point,
                                             (iface.charscreenx*gridvars.char_w) as u32, ((iface.charscreeny+1)*gridvars.char_h) as u32);

            let mut ansirect = Rect::new(x_point, y_point,
                                         gridvars.char_w as u32, gridvars.char_h as u32);

            sdl_master.canvas.set_draw_color(Color::RGBA(117, 117, 117, 255)); //Gray char select rectangle
            sdl_master.canvas.fill_rect(selectbgrect)?;
            
            for i in 33..255 { //Skipping control characters

                sdl_master.canvas.copy(&iface.ansi_char_vec[i as usize], None, ansirect)?;

                if (ansirect.x as u32 + (2*gridvars.char_w as u32)) > ((x_point as u32) + (iface.charscreenx*gridvars.char_w) as u32)  {
                    ansirect.y = ansirect.y+(gridvars.char_h as i32)+1;
                    ansirect.x = (((winwidth as i16) - iface.charscreenx*gridvars.char_w) / 2) as i32;
                }
                else {ansirect.x = ansirect.x + (gridvars.char_w as i32);}
            }

            //Rendering the character selection cursor
            //Upperline
            sdl_master.canvas.line((x_point as i16)+(iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(iface.charselector_y as i16)*(gridvars.char_h+1),
                                   (x_point as i16)+(1+iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(iface.charselector_y as i16)*(gridvars.char_h+1), curcol2)?;
            //Left line
            sdl_master.canvas.line((x_point as i16)+(iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(iface.charselector_y as i16)*(gridvars.char_h+1),
                                   (x_point as i16)+(iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1), curcol2)?;
            //Right line
            sdl_master.canvas.line((x_point as i16)+(1+iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(iface.charselector_y as i16)*(gridvars.char_h+1),
                                   (x_point as i16)+(1+iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1), curcol2)?;
            //Bottom line
            sdl_master.canvas.line((x_point as i16)+(iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1),
                                   (x_point as i16)+(1+iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1), curcol2)?;
        }
    }
    if iface.message != "" {
        //println!("{}", iface.message);
	print_message(&mut sdl_master, &iface.message)?;
        iface.message = "".to_string();
    }

    sdl_master.canvas.present();
        
    Ok(())
}

//Writes a message to the control panel for the user. Called from render_screen()
#[cfg(feature = "unsafe_textures")]
fn print_message(sdl_master: &mut SDLMasterVars, msg: &String) -> Result<(), String> {

    let tempwin = sdl_master.canvas.window_mut();
    let size = tempwin.size();
    //let winwidth = size.0;
    let winheight = size.1;

    let msg_rect = Rect::new(4, (winheight-15) as i32, (msg.len()*16) as u32, 16);

    let msgsurface = sdl_master.font.render(msg as &str).blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
    let msgtexture = sdl_master.texture_creator.create_texture_from_surface(msgsurface).map_err(|e| e.to_string())?;
    sdl_master.canvas.copy(&msgtexture, None, msg_rect)?;

    //A raw pointer to a texture is created in left brackets and then it is converted to the pointer of a type expected by the destroy method
    //Note: if msgtexture was declared as mutable, then we should have "mut" in the following line instead of "const"). However, SDL_Texture() expects mutable pointer and that is what we pass to it
    unsafe { SDL_DestroyTexture((&msgtexture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture); }
    //Aikaisemmin tuossa oli siis *mut alussakin eikä *const

    /* Less compact version of the previous line
    let tpointer1 = &mut msgtexture as *mut sdl2::render::Texture;
    let tpointer2 = tpointer1 as *mut sdl2_sys::SDL_Texture;
    SDL_DestroyTexture(tpointer2);
     */

    Ok(())
}


//This function contains some further init (continuing from main()) and then proceeds to the main loop
#[cfg(feature = "unsafe_textures")]
fn master_function(mut sdl_master: &mut SDLMasterVars, mut gridvec_obj: &mut Gridvec, mut gridvars: &mut GridMasterVars, mut iface: &mut Interface) -> Result<(), String> {

    //More init procedures
    init_ansi_textures(&mut iface, &mut sdl_master)?;
    
    update_panelbox(sdl_master, &mut iface, 1)?;
    update_panelbox(sdl_master, &mut iface, 2)?;
    update_panelbox(sdl_master, &mut iface, 3)?;
    update_panelbox(sdl_master, &mut iface, 4)?;
    update_panelbox(sdl_master, &mut iface, 5)?;

    reset_grid(&mut gridvec_obj);

    render_screen(&mut sdl_master, &mut gridvec_obj, &mut gridvars, &mut iface)?;  //run on tässä koodissa määritelty funktio

    let mut actioncode = String::from("");

    'mainloop: loop {
                    let wait_time = std::time::Duration::from_millis(10);
            std::thread::sleep(wait_time);

        for event in sdl_master.sdl_context.event_pump()?.poll_iter() {
            match event {

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if iface.program_mode == 1 {
                        if keycode == Keycode::Right {
                            //if gridvec_obj.gridvector[iface.cursor_x+1 as usize]
                            //                        if gridvec_obj.(iface.cursor_x as u16)
                            if (iface.cursor_x as u16) < gridvars.grid_x-1 {
                                //println!("Cursor x-position is {}", iface.cursor_x);
                                iface.cursor_x += 1;
                            }
                        }
                        else if keycode == Keycode::Left {
                            if iface.cursor_x > 0 {
                                iface.cursor_x -= 1;
                            }
                        }
                        else if keycode == Keycode::Down {
                            if (iface.cursor_y as u16) < (gridvars.grid_y-1) { //oli <=
                                //println!("Cursor y-position is {}", iface.cursor_y);
                                iface.cursor_y += 1;
                            }
                        }
                        else if keycode == Keycode::Up {
                            if iface.cursor_y > 0 {
                                iface.cursor_y -= 1;
                            }
                        }
                        else if keycode == Keycode::Z {if let Some(var) = iface.keys.get(&String::from("Z")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::X {if let Some(var) = iface.keys.get(&String::from("X")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::C {if let Some(var) = iface.keys.get(&String::from("C")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::V {if let Some(var) = iface.keys.get(&String::from("V")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::B {if let Some(var) = iface.keys.get(&String::from("B")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::A {if let Some(var) = iface.keys.get(&String::from("A")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::S {if let Some(var) = iface.keys.get(&String::from("S")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::D {if let Some(var) = iface.keys.get(&String::from("D")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::F {if let Some(var) = iface.keys.get(&String::from("F")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::G {if let Some(var) = iface.keys.get(&String::from("G")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::Q {if let Some(var) = iface.keys.get(&String::from("Q")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::W {if let Some(var) = iface.keys.get(&String::from("W")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::E {if let Some(var) = iface.keys.get(&String::from("E")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::R {if let Some(var) = iface.keys.get(&String::from("R")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::T {if let Some(var) = iface.keys.get(&String::from("T")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::Y {if let Some(var) = iface.keys.get(&String::from("Y")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::Backspace {if let Some(var) = iface.keys.get(&String::from("Backspace")) {actioncode = var.to_string();}}
                        else if keycode == Keycode::Num1 {if let Some(var) = iface.keys.get(&String::from("1")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num2 {if let Some(var) = iface.keys.get(&String::from("2")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num3 {if let Some(var) = iface.keys.get(&String::from("3")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num4 {if let Some(var) = iface.keys.get(&String::from("4")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num5 {if let Some(var) = iface.keys.get(&String::from("5")) {actioncode=var.to_string();}}

                        if actioncode == "PaintWithPointed" {
                            match iface.box_selector {
                                1 => actioncode = "Paint1".to_string(),
                                2 => actioncode = "Paint2".to_string(),
                                3 => actioncode = "Paint3".to_string(),
                                4 => actioncode = "Paint4".to_string(),
                                5 => actioncode = "Paint5".to_string(),
                                _ => println!("ERROR! Impossible box_selector value"), //Tähän varmaan paniikki, jos tämä kohta pitää kerran olla
                            }
                        }
                        
                        if actioncode == "Paint1" { //Painting depending on drawmode
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor1 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 1)?;
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor1 = iface.bc_selector;
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1)?;
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor1);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1)?;
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor1);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1)?;
                            }
                        }

                        else if actioncode == "Paint2" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor2 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 2)?;
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor2 = iface.bc_selector;
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2)?;
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor2);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2)?;
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor2);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2)?;
                            }
                        }

                        else if actioncode == "Paint3" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor3 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 3)?;
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor3 = iface.bc_selector;
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3)?;
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor3);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3)?;
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor3);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3)?;
                            }
                        }

                        else if actioncode == "Paint4" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor4 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 4)?;
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor4 = iface.bc_selector;
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4)?;
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor4);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4)?;
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor4);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4)?;
                            }
                        }

                        else if actioncode == "Paint5" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor5 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 5)?;
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor5 = iface.bc_selector;
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5)?;
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor5);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5)?;
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor5);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5)?;
                            }
                        }
                        
                        else if actioncode == "SelectCharColorForPointedBox" {
                            if iface.box_selector == 1 {
                                iface.fcolor1 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 1)?;
                            }
                            else if iface.box_selector == 2 {
                                iface.fcolor2 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 2)?;
                            }
                            else if iface.box_selector == 3 {
                                iface.fcolor3 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 3)?;
                            }
                            else if iface.box_selector == 4 {
                                iface.fcolor4 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 4)?;
                            }
                            else if iface.box_selector == 5 {
                                iface.fcolor5 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 5)?;
                            }
                        }
                        else if actioncode == "SelectBackColorForPointedBox" {
                            if iface.box_selector == 1 {
                                iface.bcolor1 = iface.bc_selector;
                            }
                            else if iface.box_selector == 2 {
                                iface.bcolor2 = iface.bc_selector;
                            }
                            else if iface.box_selector == 3 {
                                iface.bcolor3 = iface.bc_selector;
                            }
                            else if iface.box_selector == 4 {
                                iface.bcolor4 = iface.bc_selector;
                            }
                            else if iface.box_selector == 5 {
                                iface.bcolor5 = iface.bc_selector;
                            }                            
                        }
                        else if actioncode == "SelectCharColor" {
                            iface.message = "Select charbox for character color".to_string();
                            iface.shift = 2;
                            iface.previous_com = "SelectCharColor".to_string();
                        }
                        else if actioncode == "SelectBackgroundColor" {
                            iface.message = "Select charbox for background color".to_string();
                            iface.shift = 2;
                            iface.previous_com = "SelectBackgroundColor".to_string();
                        }
                        else if actioncode == "CharColorToBox1" { //Directly sets char color pointed by a charcolor cursor to box 1
                            iface.fcolor1 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 1)?;
                        }
                        else if actioncode == "CharColorToBox2" {
                            iface.fcolor2 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 2)?;
                        }
                        else if actioncode == "CharColorToBox3" {
                            iface.fcolor3 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 3)?;
                        }
                        else if actioncode == "CharColorToBox4" {
                            iface.fcolor4 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 4)?;
                        }
                        else if actioncode == "CharColorToBox5" {
                            iface.fcolor5 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 5)?;
                        }

                        else if actioncode == "BackgroundColorToBox1" { //Directly sets background color pointed by a background color cursor to box 1
                            iface.bcolor1 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 1)?;
                        }
                        else if actioncode == "BackgroundColorToBox2" {
                            iface.bcolor2 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 2)?;
                        }
                        else if actioncode == "BackgroundColorToBox3" {
                            iface.bcolor3 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 3)?;
                        }
                        else if actioncode == "BackgroundColorToBox4" {
                            iface.bcolor4 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 4)?;
                        }
                        else if actioncode == "BackgroundColorToBox5" {
                            iface.bcolor5 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 5)?;
                        }
                        
                        else if actioncode == "DrawmodeLeft" { //Pushing drawmode one step left
                            iface.dm_selector = iface.dm_selector-1;
                            if iface.dm_selector == 0 {iface.dm_selector = 5;}
                        }

                        else if actioncode == "DrawmodeRight" { //Pushing drawmode one step right
                            iface.dm_selector = iface.dm_selector+1;
                            if iface.dm_selector == 6 {iface.dm_selector = 1;}
                        }

                        else if actioncode == "CcolorSelectorLeft" {
                            iface.cc_selector = iface.cc_selector-1;
                            if iface.cc_selector == 0 {iface.cc_selector = 16;}
                        }
                        else if actioncode == "CcolorSelectorRight" {
                            iface.cc_selector = iface.cc_selector+1;
                            if iface.cc_selector == 17 {iface.cc_selector = 1;}
                        }

                        else if actioncode == "BcolorSelectorLeft" {
                            iface.bc_selector = iface.bc_selector-1;
                            if iface.bc_selector == 0 {iface.bc_selector = 16;}
                        }
                        else if actioncode == "BcolorSelectorRight" {
                            //println!("wir sind hier");
                            iface.bc_selector = iface.bc_selector+1;
                            if iface.bc_selector == 17 {iface.bc_selector = 1;}
                        }
                        else if actioncode == "SelectChar" { //Select char for charbox which is pointed by the charbox selector
                            iface.program_mode = 2;
                            //iface.message = "Select character by using arrow keys and space, esc to cancel".to_string();
                            iface.message = "Use arrows & space to select, esc to cancel".to_string();
//                            render_screen(font_path, &mut sdl_master, &mut gridvec_obj, &mut gridvars, &mut iface)?;
                        }

                        else if actioncode == "BoxSelectorLeft" {
                            iface.box_selector = iface.box_selector-1;
                            if iface.box_selector == 0 {iface.box_selector = 5;}
                        }
                        else if actioncode == "BoxSelectorRight" {
                            iface.box_selector = iface.box_selector+1;
                            if iface.box_selector == 6 {iface.box_selector = 1;}
                        }
                        else if actioncode == "SelectBox1" {
                            iface.box_selector = 1;
                        }
                        else if actioncode == "SelectBox2" {
                            iface.box_selector = 2;
                        }
                        else if actioncode == "SelectBox3" {
                            iface.box_selector = 3;
                        }
                        else if actioncode == "SelectBox4" {
                            iface.box_selector = 4;
                        }
                        else if actioncode == "SelectBox5" {
                            iface.box_selector = 5;
                        }

                        else if actioncode == "DeleteChar" {
                            delete_gridunit(gridvec_obj, iface.cursor_x as usize, iface.cursor_y as usize)?;
                        }
                        
                        else if actioncode == "PaintChar1" {
                            //println!("kursorin koordinaatit: {}, {}", iface.cursor_x, iface.cursor_y);
                            //println!("gridin koordinaatit: {}, {}", gridvars.grid_x, gridvars.grid_y);
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1)?;
                        }
                        else if actioncode == "PaintChar2" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2)?;
                        }
                        else if actioncode == "PaintChar3" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3)?;
                        }
                        else if actioncode == "PaintChar4" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4)?;
                        }
                        else if actioncode == "PaintChar5" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5)?;
                        }

                        else if actioncode == "ChangeBC1" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor1);
                        }
                        else if actioncode == "ChangeBC2" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor2);
                        }
                        else if actioncode == "ChangeBC3" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor3);
                        }
                        else if actioncode == "ChangeBC4" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor4);
                        }
                        else if actioncode == "ChangeBC5" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor5);
                        }

                        else if actioncode == "ChangeFC1" {
                            //Changing foreground color but not character
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1)?;
                        }
                        else if actioncode == "ChangeFC2" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2)?;
                        }
                        else if actioncode == "ChangeFC3" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3)?;
                        }
                        else if actioncode == "ChangeFC4" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4)?;
                        }
                        else if actioncode == "ChangeFC5" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5)?;
                        }

                        else if actioncode == "PaintCharNC1" {
                            //Changing character without changing its color
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                        }
                        else if actioncode == "PaintCharNC2" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                        }
                        else if actioncode == "PaintCharNC3" {
                            //Changing character without changing its color
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                        }
                        else if actioncode == "PaintCharNC4" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                        }
                        else if actioncode == "PaintCharNC5" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize)?;
                        }
                    }

                    else if iface.program_mode == 2 {
                        if keycode == Keycode::Right {
                            if (iface.charselector_x as i16) < iface.charscreenx-1 {
                                iface.charselector_x += 1;
                                //The following prevents cursor for stepping to the empty areas of selection (if 222 chars)
                                if (iface.charselector_y == 11) && (iface.charselector_x == 2) {
                                    iface.charselector_x -= 1;
                                }
                            }
                        }
                        else if keycode == Keycode::Left {
                            if iface.charselector_x > 0 {
                                iface.charselector_x -= 1;
                            }
                        }
                        else if keycode == Keycode::Down {
                            if (iface.charselector_y as i16) < (iface.charscreeny-1) {
                                if (iface.charselector_y+1)*(iface.charscreenx as u8)+iface.charselector_x < 222 {
                                    //222 = number of ANSI chars on the selector screen (no control characters in the beginning, no later cc)
                                    iface.charselector_y += 1;
                                }
                            }
                        }
                        else if keycode == Keycode::Up {
                            if iface.charselector_y > 0 {
                                iface.charselector_y -= 1;
                                
                            }
                        }
                        else if keycode == Keycode::Space || keycode == Keycode::RCtrl {
                            //Converting x & y values to a one dimensional vector index
                            let tableindex = iface.charselector_y*(iface.charscreenx as u8)+iface.charselector_x+33;
                            println!("{}", iface.charselector_y);
                            if let Some(selchar) = iface.chartable.get(&tableindex) {
                                println!("{}", selchar);
                                if iface.box_selector == 1 {
                                    iface.char1 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 1)?;
                                }
                                else if iface.box_selector == 2 {
                                    iface.char2 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 2)?;
                                }
                                else if iface.box_selector == 3 {
                                    iface.char3 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 3)?;
                                }
                                else if iface.box_selector == 4 {
                                    iface.char4 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 4)?;
                                }
                                else if iface.box_selector == 5 {
                                    iface.char5 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 5)?;
                                }
                            }
                            iface.program_mode = 1;
                        }
                        else if keycode == Keycode::Escape {
                            iface.program_mode = 1;
                        }
                    }
                    if iface.shift > 0 {iface.shift = iface.shift-1;}
                    actioncode = String::from("");
                    render_screen(&mut sdl_master, &mut gridvec_obj, &mut gridvars, &mut iface)?;
                }

                Event::Quit {..} => break 'mainloop,
                _ => {}
            }
        }
    }
    Ok(())
}

//This function updates panelbox's character (it does not update its background color which does not need a separate update function)
#[cfg(feature = "unsafe_textures")]
fn update_panelbox(sdl_master: &SDLMasterVars, mut iface: &mut Interface, boxnum: u8) -> Result<(), String> {

    if boxnum == 1 {
        let boxslice: &str = &iface.char1;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor1) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
	    unsafe {
		if let Some(texture) = &iface.panelbox1 {
		    //In contrast to print_message() function, here we have to use &iface.panelbox1 because we cannot move texture out of Some as such, only its reference. The reference is enough to create a pointer, but because it's already a reference we use "texture" instead of "&texture" and because it's not mutable we use const pointer, but destroy method needs a mutable variable so while converting the pointer for that method, it's made mutable.
		    
		    SDL_DestroyTexture((texture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
		    /* Alternative syntax:
		    let tpointer1 = texture as *const sdl2::render::Texture;
		    let tpointer2 = tpointer1 as *mut sdl2_sys::SDL_Texture;
		    SDL_DestroyTexture(tpointer2);
		     */
		}
	    }
            iface.panelbox1 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 2 {
        let boxslice: &str = &iface.char2;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor2) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
	    unsafe {
		if let Some(texture) = &iface.panelbox2 {
		    SDL_DestroyTexture((texture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
		}
	    }
            iface.panelbox2 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 3 {
        let boxslice: &str = &iface.char3;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor3) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
	    unsafe {
		if let Some(texture) = &iface.panelbox3 {
		    SDL_DestroyTexture((texture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
		}
	    }
            iface.panelbox3 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 4 {
        let boxslice: &str = &iface.char4;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor4) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
	    unsafe {
		if let Some(texture) = &iface.panelbox4 {
		    SDL_DestroyTexture((texture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
		}
	    }
            iface.panelbox4 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 5 {
        let boxslice: &str = &iface.char5;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor5) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
	    unsafe {
		if let Some(texture) = &iface.panelbox5 {
		    SDL_DestroyTexture((texture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
		}
	    }
            iface.panelbox5 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    Ok(())
}

//Struct for each char on the grid.
//#[derive(Clone)]  //#[derive()] laittaa automaattisesti annetun traitin implementaation
#[cfg(feature = "unsafe_textures")]
struct Gridunit {
    charstring: String, //No Option<> is needed since String can be empty.
    //Charstring on string, vaikka se pitää konvertoida &str:ksi. Structin on hyvä omistaa muuttujansa 
    forecol: u8, //Vaihtoehtoisesti olisi voinut käyttää tässä u8-arvoa, jonka konvertoi sdl-coloriksi hashtablella tms.
    //Forecol ei ole option, koska sillä ei ole mitään default-arvoa rendattaessa
    backcol: Option<u8>,
    chartexture: Option<sdl2::render::Texture>, //Jos texture on None, sitten ei piirretä mitään merkkiä
}

#[cfg(feature = "unsafe_textures")]
struct Interface {
    cursor_x: i16,
    cursor_y: i16,
    charselector_x: u8,
    charselector_y: u8,
    shift: u8, //For two stage commands. 2 = shift turned on during this iteration, 1 = shift is active during this iteration, 0 = no shift. Shift variable is automatically decremented, and shift = 2 is the only manual assignment
    box_selector: u8, //char box selector
    cc_selector: u8, //character color
    bc_selector: u8, //background color
    dm_selector: u8, //drawmode
    fcolor1: u8,
    fcolor2: u8,
    fcolor3: u8,
    fcolor4: u8,
    fcolor5: u8,
    bcolor1: u8,
    bcolor2: u8,
    bcolor3: u8,
    bcolor4: u8,
    bcolor5: u8,
    panelbox1: Option<sdl2::render::Texture>,
    panelbox2: Option<sdl2::render::Texture>,
    panelbox3: Option<sdl2::render::Texture>,
    panelbox4: Option<sdl2::render::Texture>,
    panelbox5: Option<sdl2::render::Texture>,
    char1: String,
    char2: String,
    char3: String,
    char4: String,
    char5: String,
    message: String,
    keys: HashMap<String, String>,
    chartable: HashMap<u8, String>,
    previous_com: String,
    program_mode: u8,
    ansi_char_vec: Vec<sdl2::render::Texture>,
    charscreenx: i16,
    charscreeny: i16,
}

#[cfg(feature = "unsafe_textures")]
struct SDLMasterVars<'a, 'b> {
    sdl_context: sdl2::Sdl,
    video_subsys: sdl2::VideoSubsystem, //oli i32
//    image_context: u8;
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: sdl2::ttf::Font<'a, 'b>,
    colortable: HashMap<u8, sdl2::pixels::Color>,
}

//Tuleeko erikseen värinvaihtofunktio vai riittääkö tämä, missä molemmat vaihtuu?
//This function changes the texture of a given gridunit to correspond its charstring
#[cfg(feature = "unsafe_textures")]
fn change_gridunit_texture<'a>(gridarg: &mut Gridvec, arg1: &'a mut SDLMasterVars, x: usize, y: usize, new_color: u8) -> Result<(), String> {
    gridarg.gridvector[x][y].forecol = new_color; //Note: Updating unit's colorcode here is a separate process to coloring a new texture below
    let charslice: &str = &gridarg.gridvector[x][y].charstring; //Converting a String to a string slice
    if let Some(color) = arg1.colortable.get(&new_color) {
        let charsurface = arg1.font.render(charslice).blended(color.clone()).map_err(|e| e.to_string())?;
	unsafe {
	    if let Some(ctexture) = &gridarg.gridvector[x][y].chartexture {
		SDL_DestroyTexture((ctexture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
	    }
	}
        gridarg.gridvector[x][y].chartexture = Some(arg1.texture_creator.create_texture_from_surface(charsurface).map_err(|e| e.to_string())? );
    }
    Ok(())
}

//Changes only the character of a gridunit texture without changing its color
#[cfg(feature = "unsafe_textures")]
fn change_gridunit_char<'a>(gridarg: &mut Gridvec, arg1: &'a mut SDLMasterVars, x: usize, y: usize) -> Result<(), String> {
    let charslice: &str = &gridarg.gridvector[x][y].charstring;
    if let Some(color) = arg1.colortable.get(&gridarg.gridvector[x][y].forecol) {
        let charsurface = arg1.font.render(charslice).blended(color.clone()).map_err(|e| e.to_string())?;
	unsafe {
	    if let Some(ctexture) = &gridarg.gridvector[x][y].chartexture {
		SDL_DestroyTexture((ctexture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
	    }
	}
        gridarg.gridvector[x][y].chartexture = Some(arg1.texture_creator.create_texture_from_surface(charsurface).map_err(|e| e.to_string())? );
    }
    Ok(())
}

//When the user deletes (cleans) a character
#[cfg(feature = "unsafe_textures")]
fn delete_gridunit<'a>(gridarg: &mut Gridvec, x: usize, y: usize) -> Result<(), String> {
    gridarg.gridvector[x][y].forecol = 0;
    gridarg.gridvector[x][y].backcol = None;
    gridarg.gridvector[x][y].charstring = "".to_string();
    unsafe {
	if let Some(texture) = &gridarg.gridvector[x][y].chartexture {
	    SDL_DestroyTexture((texture as *const sdl2::render::Texture) as *mut sdl2_sys::SDL_Texture);
	}
    }
    gridarg.gridvector[x][y].chartexture = None;
    
    Ok(())
}


//A function which fills ansi_char_vec with textures of all ansi characters
#[cfg(feature = "unsafe_textures")]
fn init_ansi_textures(iface: &mut Interface, sdl_arg: &mut SDLMasterVars) -> Result<(), String> {
    for i in 0..255 {
        //if i == 0 || i == 32 || i == 255 {continue;}
        if let Some(getchar) = iface.chartable.get(&i) {
            //println!("{}", getchar);
            let ansichar: &str = &getchar;
            let ansisurface = sdl_arg.font.render(ansichar).blended(Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string())?;
            let ansitexture = sdl_arg.texture_creator.create_texture_from_surface(ansisurface).map_err(|e| e.to_string())?;
            iface.ansi_char_vec.push(ansitexture);
            //These textures will live as long as the program is running so they won't be explicitely destroyed
        }
    }
    Ok(())
}

// kun oli palautusarvo, sulkujen jälkeen tuli -> Vec<Gridunit> {
#[cfg(feature = "unsafe_textures")]
fn reset_grid(arg: &mut Gridvec) {
    //    unsafe fn reset_grid(mut self) -> Gridvec {
    //    fn reset_grid(grid_arg: &mut Gridvec) {

    let mut counter1 = 0usize;
    let mut counter2 = 0usize;

    while counter1 < arg.gridvector.len() {
        while counter2 < arg.gridvector[counter1].len() {
            arg.gridvector[counter1][counter2].charstring = "".to_string();
            arg.gridvector[counter1][counter2].forecol = 1;
            arg.gridvector[counter1][counter2].backcol = None;

            /*
            if let Some(ctext) = arg.gridvector[counter1][counter2].chartexture {
                ctext.destroy();
            }
*/

            arg.gridvector[counter1][counter2].chartexture = None; //Tässä ilmeisesti tekstuuri vapautetaan automaattisesti
            counter2 += 1;
        }
        counter2 = 0;
        counter1 += 1;
    }
}

#[cfg(feature = "unsafe_textures")]
struct GridMasterVars {
    //Grid size
    grid_x: u16, //These values are one-based whereas gridvector and cursor values from the interface struct are zero-based
    grid_y: u16,
    //Character size
    char_w: i16,
    char_h: i16,
    //Default background color
    background_color: u8,
}

//Gridvector needs its own struct because it has to be initialised with values from GridmasterVars and thus it cannot be initialised at the same time as them
#[cfg(feature = "unsafe_textures")]
struct Gridvec {
    gridvector: Vec<Vec<Gridunit>>,  //A two dimensional vector, x & y
}

//Renders grid (and its borders) by going through gridunits of the gridvector and drawing background color tiles and characters (when found)
//Character rendering is done elsewhere, it needs not to be done all the time.
#[cfg(feature = "unsafe_textures")]
fn draw_grid(arg1: &mut GridMasterVars, arg2: &mut Gridvec, arg3: &mut SDLMasterVars) -> Result<(), String> {  //ILMEISESTI tähän ei tule structin lifetimejä, kun niitä ei käytetä

    //Borders
    //Tämän kohdan voisi optimoida siten, että laitetaan struct interfaceen nuo x ja y, jotka lasketaan alussa sekä aina kuvan kokoa muuttaessa
    let x = arg1.char_w * (arg2.gridvector.len()) as i16;
    let y = arg1.char_h * (arg2.gridvector[0].len()) as i16;

    let color = Color::RGBA(0, 182, 182, 255); //Cyan
    //i16
    arg3.canvas.line(0, y, x, y, color)?;
    arg3.canvas.line(x, 0, x, y, color)?;
    
    let mut counter1 = 0usize;
    let mut counter2 = 0usize;
    
    while counter1 < arg2.gridvector.len() {
        while counter2 < arg2.gridvector[counter1].len() {  //Kaikki rivit on samanpituisia, eikö counter1:n voisi korvata esim 0:lla

            let unitrect = Rect::new((counter1 as i32)*arg1.char_w as i32, (counter2 as i32)*arg1.char_h as i32, arg1.char_w as u32, arg1.char_h as u32);

            //If the background color for a grid unit is not set, GridMasterVars's default background color is used
            if let Some(bc) = arg2.gridvector[counter1][counter2].backcol {
                if let Some(color) = arg3.colortable.get(&bc) {
                    arg3.canvas.set_draw_color(color.clone());
                }
            }
            else {
                if let Some(color) = arg3.colortable.get(&arg1.background_color) {
                    arg3.canvas.set_draw_color(color.clone());
                }
            }

            arg3.canvas.fill_rect(unitrect)?;

            //If the grid unit has a texture, it will be rendered
            if let Some(ctext) = arg2.gridvector[counter1][counter2].chartexture.as_ref() {
                arg3.canvas.copy(ctext, None, Some(unitrect))?; //Mikä toinen parametri on?
            }
            
            counter2 += 1;
        }
        counter2 = 0;
        counter1 += 1;
    }

    Ok(())
}


#[cfg(feature = "unsafe_textures")]
fn main() -> Result<(), String> {

    let args: Vec<_> = env::args().collect();
    //env-moduulin args (env::args() on iteraattori prosessin (ilmeisesti komentorivi-)argumenttien yli, palauttaa kustakin stringin
    //Tässä luodaan siis vektori iteroimalla, jolloin käytetään collect():ia. Tässä tapauksessa kerätään prosessin argumentit.
    //tuossa Vec<_> on yleensä datatyyppi alaviivan tilalla, mutta ilmeisesti sitä ei tarvita

    if args.len() < 2 {
        println!("Give a path to the font file as an argument");
        std::process::exit(1)
    }

    let path: &Path = Path::new(&args[1]);

    let default_grid_w = 50u16; //50 tässä antaa gridveciin ja interfaceen rangen 0-49 eli tämä arvo on one-based
    let default_grid_h = 20u16; //Sama kuin yllä
    let default_char_w = 8i16; //oli 16, myöhemmin arvot olivat 7 & 14
    let default_char_h = 16i16; //oli 16

        //Path on tiedostopolkua käsittelevä tyyppi, jossa on sen käsittelyä helpottavia operaatioita. Tyyppi on unsized, joten pitää käyttää pointteria, kuten & tai Box
        //new luo Pathin string slicestä. 

    //Tässä siis SDLMasterVars -structin muuttujat luodaan ensin ja sitten vasta laitetaan sinne itse structiin, kun structin instanssi vihdoin luodaan. Jos yritti ensin luoda instanssin, piti käyttää Option<T> -muuttujia, jotka sotkee ohjelman toiminnan niin, ettei alustettuihin muuttujiin kuuluvia metodeita (esim. sdl_context.video()) sitten löydykään. Ehkä myös jotkut kääntäjän ehdottamat phantom variablet olisi voinut laittaa tuohon structin muuttujien datatyypeiksi optionin sijaan, mutta jäi silti kuva, että vain structin ulkopuolella alustaen nuo muuttujat voivat toimia. 
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
//    let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    //+50 is the space for interface's panel
    let window = video_subsys
        //ikkunan koko
        //.window("LammasDRAW", (default_grid_w*(default_char_w as u16)) as u32, (default_grid_h*(default_char_h as u16)+50) as u32)
        .window("LammasDRAW", 800, 600)
        //.resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font(path, 64)?; //Toinen argumentti on fonttikoko
    
    let mut sdl_masterobj = SDLMasterVars {sdl_context, video_subsys, canvas, texture_creator, font, colortable: HashMap::new()};
    /*
    Colortable:
    1 = black
    2 = white
    3 = red
    4 = yellow
    5 = light blue
    6 = dark blue
    7 = light green
    8 = dark green
    9 = light purple
    10 = dark purple
    11 = cyan??
    12 = light cyan
    13 = dark cyan
    14 = brown
    15 = light gray
    16 = dark gray
     */
    sdl_masterobj.colortable.insert(1, Color::RGBA(0, 0, 0, 255));
    sdl_masterobj.colortable.insert(2, Color::RGBA(255, 255, 255, 255));
    sdl_masterobj.colortable.insert(3, Color::RGBA(252, 84, 84, 255));
    sdl_masterobj.colortable.insert(4, Color::RGBA(252, 253, 87, 255));
    sdl_masterobj.colortable.insert(5, Color::RGBA(85, 85, 255, 255));
    sdl_masterobj.colortable.insert(6, Color::RGBA(0, 0, 170, 255));
    sdl_masterobj.colortable.insert(7, Color::RGBA(86, 252, 88, 255));
    sdl_masterobj.colortable.insert(8, Color::RGBA(0, 170, 0, 255));
    sdl_masterobj.colortable.insert(9, Color::RGBA(252, 84, 252, 255));
    sdl_masterobj.colortable.insert(10, Color::RGBA(168, 0, 168, 255));
    sdl_masterobj.colortable.insert(11, Color::RGBA(0, 170, 170, 255));
    sdl_masterobj.colortable.insert(12, Color::RGBA(64, 228, 226, 255));
    sdl_masterobj.colortable.insert(13, Color::RGBA(42, 129, 128, 255));
    sdl_masterobj.colortable.insert(14, Color::RGBA(168, 84, 0, 255));
    sdl_masterobj.colortable.insert(15, Color::RGBA(171, 173, 170, 255));
    sdl_masterobj.colortable.insert(16, Color::RGBA(85, 86, 85, 255));

    let mut interfaceobj = Interface {cursor_x: 0,  cursor_y: 0, charselector_x: 0, charselector_y: 0,
                                      shift: 0, box_selector: 1, cc_selector: 1, bc_selector: 1, dm_selector: 1,
                                      fcolor1: 3, fcolor2: 4, fcolor3: 5, fcolor4: 6, fcolor5: 7,
                                      bcolor1: 4, bcolor2: 1, bcolor3: 2, bcolor4: 15, bcolor5: 16,
                                      panelbox1: None, panelbox2: None, panelbox3: None, panelbox4: None, panelbox5: None,
                                      char1: "#".to_string(), char2: "▓".to_string(), char3: "▒".to_string(),
                                      char4: "░".to_string(), char5: "█".to_string(), message: "".to_string(),
                                      keys: HashMap::new(), chartable: HashMap::new(), previous_com: "".to_string(),
                                      program_mode: 1, ansi_char_vec: vec![], charscreenx: 20, charscreeny: 12};

    /*
    interfaceobj.keys.insert(String::from("Z"), String::from("Paint1"));
    interfaceobj.keys.insert(String::from("X"), String::from("Paint2"));
    interfaceobj.keys.insert(String::from("C"), String::from("Paint3"));
    interfaceobj.keys.insert(String::from("V"), String::from("Paint4"));
    interfaceobj.keys.insert(String::from("B"), String::from("Paint5"));
    */

    interfaceobj.keys.insert(String::from("A"), String::from("DrawmodeLeft"));
    interfaceobj.keys.insert(String::from("S"), String::from("DrawmodeRight"));
    interfaceobj.keys.insert(String::from("E"), String::from("CcolorSelectorLeft"));
    interfaceobj.keys.insert(String::from("R"), String::from("CcolorSelectorRight"));
    interfaceobj.keys.insert(String::from("Q"), String::from("BcolorSelectorLeft"));
    interfaceobj.keys.insert(String::from("W"), String::from("BcolorSelectorRight"));
//    interfaceobj.keys.insert(String::from("T"), String::from("SelectCharColor"));
//    interfaceobj.keys.insert(String::from("G"), String::from("SelectBackgroundColor"));
    interfaceobj.keys.insert(String::from("D"), String::from("BoxSelectorLeft"));
    interfaceobj.keys.insert(String::from("F"), String::from("BoxSelectorRight"));
    interfaceobj.keys.insert(String::from("V"), String::from("SelectChar"));
    interfaceobj.keys.insert(String::from("T"), String::from("SelectCharColorForPointedBox"));
    interfaceobj.keys.insert(String::from("Y"), String::from("SelectBackColorForPointedBox"));
    interfaceobj.keys.insert(String::from("G"), String::from("PaintWithPointed"));
    interfaceobj.keys.insert(String::from("Backspace"), String::from("DeleteChar"));

    //NOTE: The current font has no characters 0-32 and they are not used. If they are taken into use, this will have some implications for the code, especially for some loops plus for the controls of the character selection cursor
    interfaceobj.chartable.insert(0, String::from(" ")); //Control char
    interfaceobj.chartable.insert(1, String::from("☺"));
    interfaceobj.chartable.insert(2, String::from("☻"));
    interfaceobj.chartable.insert(3, String::from("♥")); 
    interfaceobj.chartable.insert(4, String::from("♦"));
    interfaceobj.chartable.insert(5, String::from("♣"));
    interfaceobj.chartable.insert(6, String::from("♠"));
    interfaceobj.chartable.insert(7, String::from("•"));
    interfaceobj.chartable.insert(8, String::from("◘"));
    interfaceobj.chartable.insert(9, String::from("○"));
    interfaceobj.chartable.insert(10, String::from("◙"));
    interfaceobj.chartable.insert(11, String::from("♂"));
    interfaceobj.chartable.insert(12, String::from("♀"));
    interfaceobj.chartable.insert(13, String::from("♪"));
    interfaceobj.chartable.insert(14, String::from("♫"));
    interfaceobj.chartable.insert(15, String::from("☼"));
    interfaceobj.chartable.insert(16, String::from("►"));
    interfaceobj.chartable.insert(17, String::from("◄"));
    interfaceobj.chartable.insert(18, String::from("↕"));
    interfaceobj.chartable.insert(19, String::from("‼"));
    interfaceobj.chartable.insert(20, String::from("¶"));
    interfaceobj.chartable.insert(21, String::from("§"));
    interfaceobj.chartable.insert(22, String::from("▬"));
    interfaceobj.chartable.insert(23, String::from("↨"));
    interfaceobj.chartable.insert(24, String::from("↑"));
    interfaceobj.chartable.insert(25, String::from("↓"));
    interfaceobj.chartable.insert(26, String::from("→"));
    interfaceobj.chartable.insert(27, String::from("←"));
    interfaceobj.chartable.insert(28, String::from("∟"));
    interfaceobj.chartable.insert(29, String::from("↔"));
    interfaceobj.chartable.insert(30, String::from("▲"));
    interfaceobj.chartable.insert(31, String::from("▼"));
    interfaceobj.chartable.insert(32, String::from(" ")); //Control char
    interfaceobj.chartable.insert(33, String::from("!"));
    interfaceobj.chartable.insert(34, String::from("\"")); //Character "
    interfaceobj.chartable.insert(35, String::from("#"));
    interfaceobj.chartable.insert(36, String::from("$"));
    interfaceobj.chartable.insert(37, String::from("%"));
    interfaceobj.chartable.insert(38, String::from("&"));
    interfaceobj.chartable.insert(39, String::from("'"));
    interfaceobj.chartable.insert(40, String::from("("));
    interfaceobj.chartable.insert(41, String::from(")"));
    interfaceobj.chartable.insert(42, String::from("*"));
    interfaceobj.chartable.insert(43, String::from("+"));
    interfaceobj.chartable.insert(44, String::from(","));
    interfaceobj.chartable.insert(45, String::from("-"));
    interfaceobj.chartable.insert(46, String::from("."));
    interfaceobj.chartable.insert(47, String::from("/"));
    interfaceobj.chartable.insert(48, String::from("0"));
    interfaceobj.chartable.insert(49, String::from("1"));
    interfaceobj.chartable.insert(50, String::from("2"));
    interfaceobj.chartable.insert(51, String::from("3"));
    interfaceobj.chartable.insert(52, String::from("4"));
    interfaceobj.chartable.insert(53, String::from("5"));
    interfaceobj.chartable.insert(54, String::from("6"));
    interfaceobj.chartable.insert(55, String::from("7"));
    interfaceobj.chartable.insert(56, String::from("8"));
    interfaceobj.chartable.insert(57, String::from("9"));
    interfaceobj.chartable.insert(58, String::from(":"));
    interfaceobj.chartable.insert(59, String::from(";"));
    interfaceobj.chartable.insert(60, String::from("<"));
    interfaceobj.chartable.insert(61, String::from("="));
    interfaceobj.chartable.insert(62, String::from(">"));
    interfaceobj.chartable.insert(63, String::from("?"));
    interfaceobj.chartable.insert(64, String::from("@"));
    interfaceobj.chartable.insert(65, String::from("A"));
    interfaceobj.chartable.insert(66, String::from("B"));
    interfaceobj.chartable.insert(67, String::from("C"));
    interfaceobj.chartable.insert(68, String::from("D"));
    interfaceobj.chartable.insert(69, String::from("E"));
    interfaceobj.chartable.insert(70, String::from("F"));
    interfaceobj.chartable.insert(71, String::from("G"));
    interfaceobj.chartable.insert(72, String::from("H"));
    interfaceobj.chartable.insert(73, String::from("I"));
    interfaceobj.chartable.insert(74, String::from("J"));
    interfaceobj.chartable.insert(75, String::from("K"));
    interfaceobj.chartable.insert(76, String::from("L"));
    interfaceobj.chartable.insert(77, String::from("M"));
    interfaceobj.chartable.insert(78, String::from("N"));
    interfaceobj.chartable.insert(79, String::from("O"));
    interfaceobj.chartable.insert(80, String::from("P"));
    interfaceobj.chartable.insert(81, String::from("Q"));
    interfaceobj.chartable.insert(82, String::from("R"));
    interfaceobj.chartable.insert(83, String::from("S"));
    interfaceobj.chartable.insert(84, String::from("T"));
    interfaceobj.chartable.insert(85, String::from("U"));
    interfaceobj.chartable.insert(86, String::from("V"));
    interfaceobj.chartable.insert(87, String::from("W"));
    interfaceobj.chartable.insert(88, String::from("X"));
    interfaceobj.chartable.insert(89, String::from("Y"));
    interfaceobj.chartable.insert(90, String::from("Z"));
    interfaceobj.chartable.insert(91, String::from("["));
    interfaceobj.chartable.insert(92, String::from("\\")); //character \ needs an escape backslash
    interfaceobj.chartable.insert(93, String::from("]"));
    interfaceobj.chartable.insert(94, String::from("^"));
    interfaceobj.chartable.insert(95, String::from("_"));
    interfaceobj.chartable.insert(96, String::from("`"));
    interfaceobj.chartable.insert(97, String::from("a"));
    interfaceobj.chartable.insert(98, String::from("b"));
    interfaceobj.chartable.insert(99, String::from("c"));
    interfaceobj.chartable.insert(100, String::from("d"));
    interfaceobj.chartable.insert(101, String::from("e"));
    interfaceobj.chartable.insert(102, String::from("f"));
    interfaceobj.chartable.insert(103, String::from("g"));
    interfaceobj.chartable.insert(104, String::from("h"));
    interfaceobj.chartable.insert(105, String::from("i"));
    interfaceobj.chartable.insert(106, String::from("j"));
    interfaceobj.chartable.insert(107, String::from("k"));
    interfaceobj.chartable.insert(108, String::from("l"));
    interfaceobj.chartable.insert(109, String::from("m"));
    interfaceobj.chartable.insert(110, String::from("n"));
    interfaceobj.chartable.insert(111, String::from("o"));
    interfaceobj.chartable.insert(112, String::from("p"));
    interfaceobj.chartable.insert(113, String::from("q"));
    interfaceobj.chartable.insert(114, String::from("r"));
    interfaceobj.chartable.insert(115, String::from("s"));
    interfaceobj.chartable.insert(116, String::from("t"));
    interfaceobj.chartable.insert(117, String::from("u"));
    interfaceobj.chartable.insert(118, String::from("v"));
    interfaceobj.chartable.insert(119, String::from("w"));
    interfaceobj.chartable.insert(120, String::from("x"));
    interfaceobj.chartable.insert(121, String::from("y"));
    interfaceobj.chartable.insert(122, String::from("z"));
    interfaceobj.chartable.insert(123, String::from("{"));
    interfaceobj.chartable.insert(124, String::from("|"));
    interfaceobj.chartable.insert(125, String::from("}"));
    interfaceobj.chartable.insert(126, String::from("~"));
    interfaceobj.chartable.insert(127, String::from("⌂"));
    interfaceobj.chartable.insert(128, String::from("Ç"));
    interfaceobj.chartable.insert(129, String::from("ü"));
    interfaceobj.chartable.insert(130, String::from("é"));
    interfaceobj.chartable.insert(131, String::from("â"));
    interfaceobj.chartable.insert(132, String::from("ä"));
    interfaceobj.chartable.insert(133, String::from("à"));
    interfaceobj.chartable.insert(134, String::from("å"));
    interfaceobj.chartable.insert(135, String::from("ç"));
    interfaceobj.chartable.insert(136, String::from("ê"));
    interfaceobj.chartable.insert(137, String::from("ë"));
    interfaceobj.chartable.insert(138, String::from("è"));
    interfaceobj.chartable.insert(139, String::from("ï"));
    interfaceobj.chartable.insert(140, String::from("î"));
    interfaceobj.chartable.insert(141, String::from("ì"));
    interfaceobj.chartable.insert(142, String::from("Ä"));
    interfaceobj.chartable.insert(143, String::from("Å"));
    interfaceobj.chartable.insert(144, String::from("É"));
    interfaceobj.chartable.insert(145, String::from("æ"));
    interfaceobj.chartable.insert(146, String::from("Æ"));
    interfaceobj.chartable.insert(147, String::from("ô"));
    interfaceobj.chartable.insert(148, String::from("ö"));
    interfaceobj.chartable.insert(149, String::from("ò"));
    interfaceobj.chartable.insert(150, String::from("û"));
    interfaceobj.chartable.insert(151, String::from("ù"));
    interfaceobj.chartable.insert(152, String::from("ÿ"));
    interfaceobj.chartable.insert(153, String::from("Ö"));
    interfaceobj.chartable.insert(154, String::from("Ü"));
    interfaceobj.chartable.insert(155, String::from("¢"));
    interfaceobj.chartable.insert(156, String::from("£"));
    interfaceobj.chartable.insert(157, String::from("¥"));
    interfaceobj.chartable.insert(158, String::from("₧"));
    interfaceobj.chartable.insert(159, String::from("ƒ"));
    interfaceobj.chartable.insert(160, String::from("á"));
    interfaceobj.chartable.insert(161, String::from("í"));
    interfaceobj.chartable.insert(162, String::from("ó"));
    interfaceobj.chartable.insert(163, String::from("ú"));
    interfaceobj.chartable.insert(164, String::from("ñ"));
    interfaceobj.chartable.insert(165, String::from("Ñ"));
    interfaceobj.chartable.insert(166, String::from("ª"));
    interfaceobj.chartable.insert(167, String::from("º"));
    interfaceobj.chartable.insert(168, String::from("¿"));
    interfaceobj.chartable.insert(169, String::from("⌐"));
    interfaceobj.chartable.insert(170, String::from("¬"));
    interfaceobj.chartable.insert(171, String::from("½"));
    interfaceobj.chartable.insert(172, String::from("¼"));
    interfaceobj.chartable.insert(173, String::from("¡"));
    interfaceobj.chartable.insert(174, String::from("«"));
    interfaceobj.chartable.insert(175, String::from("»"));
    interfaceobj.chartable.insert(176, String::from("░"));
    interfaceobj.chartable.insert(177, String::from("▒"));
    interfaceobj.chartable.insert(178, String::from("▓"));
    interfaceobj.chartable.insert(179, String::from("│"));
    interfaceobj.chartable.insert(180, String::from("┤"));
    interfaceobj.chartable.insert(181, String::from("╡"));
    interfaceobj.chartable.insert(182, String::from("╢"));
    interfaceobj.chartable.insert(183, String::from("╖"));
    interfaceobj.chartable.insert(184, String::from("╕"));
    interfaceobj.chartable.insert(185, String::from("╣"));
    interfaceobj.chartable.insert(186, String::from("║"));
    interfaceobj.chartable.insert(187, String::from("╗"));
    interfaceobj.chartable.insert(188, String::from("╝"));
    interfaceobj.chartable.insert(189, String::from("╜"));
    interfaceobj.chartable.insert(190, String::from("╛"));
    interfaceobj.chartable.insert(191, String::from("┐"));
    interfaceobj.chartable.insert(192, String::from("└"));
    interfaceobj.chartable.insert(193, String::from("┴"));
    interfaceobj.chartable.insert(194, String::from("┬"));
    interfaceobj.chartable.insert(195, String::from("├"));
    interfaceobj.chartable.insert(196, String::from("─"));
    interfaceobj.chartable.insert(197, String::from("┼"));
    interfaceobj.chartable.insert(198, String::from("╞"));
    interfaceobj.chartable.insert(199, String::from("╟"));
    interfaceobj.chartable.insert(200, String::from("╚"));
    interfaceobj.chartable.insert(201, String::from("╔"));
    interfaceobj.chartable.insert(202, String::from("╩"));
    interfaceobj.chartable.insert(203, String::from("╦"));
    interfaceobj.chartable.insert(204, String::from("╠"));
    interfaceobj.chartable.insert(205, String::from("═"));
    interfaceobj.chartable.insert(206, String::from("╬"));
    interfaceobj.chartable.insert(207, String::from("╧"));
    interfaceobj.chartable.insert(208, String::from("╨"));
    interfaceobj.chartable.insert(209, String::from("╤"));
    interfaceobj.chartable.insert(210, String::from("╥"));
    interfaceobj.chartable.insert(211, String::from("╙"));
    interfaceobj.chartable.insert(212, String::from("╘"));
    interfaceobj.chartable.insert(213, String::from("╒"));
    interfaceobj.chartable.insert(214, String::from("╓"));
    interfaceobj.chartable.insert(215, String::from("╫"));
    interfaceobj.chartable.insert(216, String::from("╪"));
    interfaceobj.chartable.insert(217, String::from("┘"));
    interfaceobj.chartable.insert(218, String::from("┌"));
    interfaceobj.chartable.insert(219, String::from("█"));
    interfaceobj.chartable.insert(220, String::from("▄"));
    interfaceobj.chartable.insert(221, String::from("▌"));
    interfaceobj.chartable.insert(222, String::from("▐"));
    interfaceobj.chartable.insert(223, String::from("▀"));
    interfaceobj.chartable.insert(224, String::from("α"));
    interfaceobj.chartable.insert(225, String::from("ß"));
    interfaceobj.chartable.insert(226, String::from("Γ"));
    interfaceobj.chartable.insert(227, String::from("π"));
    interfaceobj.chartable.insert(228, String::from("Σ"));
    interfaceobj.chartable.insert(229, String::from("σ"));
    interfaceobj.chartable.insert(230, String::from("µ"));
    interfaceobj.chartable.insert(231, String::from("τ"));
    interfaceobj.chartable.insert(232, String::from("Φ"));
    interfaceobj.chartable.insert(233, String::from("Θ"));
    interfaceobj.chartable.insert(234, String::from("Ω"));
    interfaceobj.chartable.insert(235, String::from("δ"));
    interfaceobj.chartable.insert(236, String::from("∞"));
    interfaceobj.chartable.insert(237, String::from("φ"));
    interfaceobj.chartable.insert(238, String::from("ε"));
    interfaceobj.chartable.insert(239, String::from("∩"));
    interfaceobj.chartable.insert(240, String::from("≡"));
    interfaceobj.chartable.insert(241, String::from("±"));
    interfaceobj.chartable.insert(242, String::from("≥"));
    interfaceobj.chartable.insert(243, String::from("≤"));
    interfaceobj.chartable.insert(244, String::from("⌠"));
    interfaceobj.chartable.insert(245, String::from("⌡"));
    interfaceobj.chartable.insert(246, String::from("÷"));
    interfaceobj.chartable.insert(247, String::from("≈"));
    interfaceobj.chartable.insert(248, String::from("°"));
    interfaceobj.chartable.insert(249, String::from("∙"));
    interfaceobj.chartable.insert(250, String::from("·"));
    interfaceobj.chartable.insert(251, String::from("√"));
    interfaceobj.chartable.insert(252, String::from("ⁿ"));
    interfaceobj.chartable.insert(253, String::from("²"));
    interfaceobj.chartable.insert(254, String::from("■"));
    interfaceobj.chartable.insert(255, String::from(" ")); //Control char

    let mut gridvars = GridMasterVars {grid_x: default_grid_w, grid_y: default_grid_h, char_w: default_char_w, char_h: default_char_h, background_color: 1};
    //    let mut gridvector_obj = Gridvec {gridvector: vec![vec![generic_gridunit; gridvars.grid_y];gridvars.grid_x] };

    //let mut gridvector_obj = Gridvec {gridvector: vec![vec![],vec![]]};
    let mut gridvector_obj = Gridvec {gridvector: vec![]};
    //Pushing inner vectors to the "carrier vector"
    for _ in 0..(gridvars.grid_x) {
        gridvector_obj.gridvector.push(vec![]);
    }
    //Pushing gridunits to inner vectors
    //Tämän voisi varmaan tehdä iteraattoreillakin??
    for i in 0..(gridvars.grid_x) {
        for _ in 0..(gridvars.grid_y) {
            gridvector_obj.gridvector[i as usize].push(Gridunit {charstring: "".to_string(), forecol: 1, backcol: None, chartexture: None});
        }
    }

    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());  //TTF:n funktio joka palauttaa version

    master_function(&mut sdl_masterobj, &mut gridvector_obj, &mut gridvars, &mut interfaceobj)?;  //run on tässä koodissa määritelty funktio

    Ok(())
}

#[cfg(not(feature = "unsafe_textures"))]
pub fn main(){}

