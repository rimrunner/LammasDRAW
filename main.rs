/*
cargo run --features unsafe_textures

-Missä mennään:
-box selector -kursorin voisi laittaa menemään nätisti
-Voisi tehdä loputkin käskyt tuota uutta käyttöliittymäratkaisua varten

-Ei ole testattu taustavärin valintaa eikä suoria värivalintakomentoja

-chartable
--toiminto, josta vaihtuu boxin 1-5 char suoraan ja toiminto, joka käyttää shiftiä
--merkin voi valita ihan vaan heittämällä ne numerot ...mutta tämähän vaatii teksti-inputin luomisen
--tai sitten aukeaa se valintaruutu. Siinä olisi tosiaan kätevä käyttää sitä taulukkoa
-char-valintaruutu
--render screen()iin program moden (interfacen muuttuja) perusteella täysin toinen haara
--eri kursorimuuttujat siihen ja kursorinapinpainalluksiin myös se program mode
--sitten se valintaruudun piirto

-Värin valinta: oma nappi cc ja bc värien valinnalle ja sitten menee värinvalinta päälle
-Sitten voi tehdä myös toiminnot, että suoraan valitse cc/bc charboxiin 1 jne.

-Värin valinta charboxeihin
-Merkin valinta charboxeihin
-Ei-taustavärejä ei pitäisi voida valita taustaväreiksi charboxeihin
-Delete char tarvitaan
-pitäisi tehdä se tekstuurin tuhoaminen. mutta miten se onnistuu?
-erikseen paneelin piirto ja kuvan piirto. onnistuuko?
-render screen() -kutsuja pitäisi saada pois
-rectit ja muut paneelin piirtämisestä muuanne?
-koodinaatitkin (zero-based, kuten grafx2:ssa) varmaan näkyviin paneeliin
-pitäisikö indekseihin mennä aina varovaisella get-menetelmällä?
-ohjelmassa voi piirtää tyhjään ruutuun pelkän taustavärin, mutta se pitäisi kai säilyttää ruudun täyttävänä palkkina? Mutta hetkinen. Tuleeko tässä ongelma, jos taustavärit on eri värejä? Ei tule, jos taustavärit on samoja, kuin mitä muut värit ovat.
-Onko ansiartissa vakiovärit vai pitääkö tehdä värinvalinta

-PULMA: kun piirsi pelkän charin (drawmode: "char") tyhjälle ruudulle, se ehkä piirtyi joko mustaksi (oliko joku musta oletusväri päällä?) tai sitten muutti pelkän gridunitin char-muuttujan eikä muuta. mitäköhän tässä pitäisi tapahtua? Ehkä ei saa olla mahdollista, että charcolor ja bgcolor on sama, koska muutenhan voi niitä jäädä pimentoon ja niitä sitten seivaillaan turhaan. Joo, pitää tehdä tarkistus tuohon, että onko värit samat.

https://rust-sdl2.github.io/rust-sdl2/sdl2/keyboard/enum.Keycode.html

--

Selectors:
select drawmode: N, M
drawmodes: all, char & charcolor, only char, only background, only charcolor
draw contents of drawbox 1-5: Z-B
move draw box selector: A, S
move character color selector: D, F
move background color selector: G, H
select pointed character color to a pointed drawbox:
select pointed background color to a pointed drawbox:

--
-gridvarsin arvot ja default_grid_w/h ovat käytännössä ilmeisesti one-based, kun taas gridvec ja interface (kursori) ovat zero-based
-kun noilla one-based arvoilla alustetaan ikkuna ja gridvec (ilman mitään muunnoksia) tulee niihin sitten zero-based (vrt. kun alustat arrayn 5:llä, tulee arrayn indekseiksi 0-4)
-esim kun gridvarseilla rajataan kursorin liike, siinä näkyy tuo muunnos


Värit
musta 0,0,0
valkoinen 255,255,255
cyan/magneto 0,170,170
kirkas cyan 64,228,226
tumma cyan 42,129,128
harmaa 85,86,85
vaaleanharmaa 171,173,170
vihreä 86,252,88
tummempi vihreä 0,170,0
keltainen 252,253,87
punainen 252, 84, 84
violetti 168,0,168
vaaleampi violetti 252,84,252
ruskea 168,84,0
vaaleansininen 85, 85, 255
tummansininen 0,0,170

Taustavärit
musta
tummempi harmaa
tummansininen
tummempi vihreä
tummempi syaani
?
?
?

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
use std::collections::HashMap;

#[cfg(feature = "unsafe_textures")]
static SCREEN_WIDTH : u32 = 800;
#[cfg(feature = "unsafe_textures")]
static SCREEN_HEIGHT : u32 = 600;

// handle the annoying Rect i32
#[cfg(feature = "unsafe_textures")]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
#[cfg(feature = "unsafe_textures")]
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

/*
trait Color_conv {
    fn conv(&self) -> Color {
        SDL_Color::From<&sdl2_sys::SDL_Color>
    }
}
*/

#[cfg(feature = "unsafe_textures")]
fn render_screen(font_path: &Path, mut sdl_master: &mut SDLMasterVars, mut gridvec_obj: &mut Gridvec, mut gridvars: &mut GridMasterVars, mut iface: &mut Interface) -> Result<(), String> {

    //TESTI:
    /*
    gridvec_obj.gridvector[2][2].charstring = "┤".to_string();
    change_gridunit_texture(&mut gridvec_obj, &mut sdl_master, 2, 2, Color::RGBA(255, 0, 0, 255));
    gridvec_obj.gridvector[3][3].charstring = "Y".to_string();
    gridvec_obj.gridvector[3][3].backcol = Some(Color::RGBA(255, 255, 255, 255));
    change_gridunit_texture(&mut gridvec_obj, &mut sdl_master, 3, 3, Color::RGBA(100, 200, 0, 255));
     */
    
    //    let sdl_context = sdl2::init()?;   //Kysymysmerkki purkaa Resultin eli tässä tapauksessa palauttaa Stringin jos tulee virhe
    //let video_subsys = SDL_object.sdl_context.video()?;
    //    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    /*
    let mut window = video_subsys.window("LammasDRAW", SCREEN_WIDTH, SCREEN_HEIGHT)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;
     */

    //    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    //    let texture_creator = canvas.texture_creator();

    // Load a font
    //    let font = ttf_context.load_font(font_path, 64)?; //Toinen argumentti on fonttikoko
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas

    /*
    let surface = sdl_master.font.render("Hello┤ Rust!")
        .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;
    let texture = sdl_master.texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
*/

    sdl_master.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    sdl_master.canvas.clear();

    let tempwin = sdl_master.canvas.window_mut();
    let size = tempwin.size();
    let winwidth = size.0;
    let winheight = size.1;

    //Cursor colors
    let curcol1 = Color::RGBA(100, 100, 100, 255); //Gray
    let curcol2 = Color::RGBA(255, 0, 0, 255); //Red

    //let sdl2::render::TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)

    //let padding = 64;
    //let target = get_centered_rect(width, height, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

    if iface.program_mode == 1 || iface.program_mode == 2 { //1 = Normal drawing mode, 2 = ANSI char select screen
    
        //Pitäisikö tämä siirtää interfaceen?
        let panel = Rect::new(0 as i32, (winheight-100) as i32, (gridvars.char_w*(gridvars.grid_x as i16)) as u32, winheight as u32);
        //SDL_SetRenderDrawColor(piirturi, 0xFF, 0xA5, 0x00, 0x00);
        sdl_master.canvas.set_draw_color(Color::RGBA(130, 130, 130, 255));
        sdl_master.canvas.fill_rect(panel)?;

        //Rendering palette
        let mut palrect = Rect::new((winwidth-16) as i32, (winheight-26) as i32, 16, 16);
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
        let charbox1 = Rect::new(10, (gridvars.char_w*(gridvars.grid_y as i16)) as i32, gridvars.char_w as u32, gridvars.char_h as u32);
        let charbox2 = Rect::new(30, (gridvars.char_w*(gridvars.grid_y as i16)) as i32, gridvars.char_w as u32, gridvars.char_h as u32);
        let charbox3 = Rect::new(50, (gridvars.char_w*(gridvars.grid_y as i16)) as i32, gridvars.char_w as u32, gridvars.char_h as u32);
        let charbox4 = Rect::new(70, (gridvars.char_w*(gridvars.grid_y as i16)) as i32, gridvars.char_w as u32, gridvars.char_h as u32);
        let charbox5 = Rect::new(90, (gridvars.char_w*(gridvars.grid_y as i16)) as i32, gridvars.char_w as u32, gridvars.char_h as u32);

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

        if let Some(pb1) = iface.panelbox1.as_ref() {sdl_master.canvas.copy(pb1, None, charbox1);}
        if let Some(pb2) = iface.panelbox2.as_ref() {sdl_master.canvas.copy(pb2, None, charbox2);}
        if let Some(pb3) = iface.panelbox3.as_ref() {sdl_master.canvas.copy(pb3, None, charbox3);}
        if let Some(pb4) = iface.panelbox4.as_ref() {sdl_master.canvas.copy(pb4, None, charbox4);}
        if let Some(pb5) = iface.panelbox5.as_ref() {sdl_master.canvas.copy(pb5, None, charbox5);}

        let drawmode_surface = sdl_master.font.render("|ALL | CHAR+COL | CHAR | BG | CHARCOL|").blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
        let drawmode_text = sdl_master.texture_creator.create_texture_from_surface(drawmode_surface).map_err(|e| e.to_string())? ;
        let dmrect = Rect::new(120, (gridvars.char_w*(gridvars.grid_y as i16)) as i32, 191, 17);
        sdl_master.canvas.copy(&drawmode_text, None, dmrect)?;

        //Drawmode pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        let mut dmselrect = Rect::new(126, ((gridvars.char_w*(gridvars.grid_y as i16)+14 as i16)) as i32, 10, 4);
        if iface.dm_selector == 1 {dmselrect.x = dmselrect.x + 5;}
        else if iface.dm_selector == 2 {dmselrect.x = dmselrect.x + 40;}
        else if iface.dm_selector == 3 {dmselrect.x = dmselrect.x + 80;}
        else if iface.dm_selector == 4 {dmselrect.x = dmselrect.x + 120;}
        else if iface.dm_selector == 5 {dmselrect.x = dmselrect.x + 160;}
        sdl_master.canvas.fill_rect(dmselrect)?;

        /*
        if let Some(dm_image) = iface.drawmode_img {
        //Tässä on nyt 191 ja 17 katsottu png-tiedostosta, pitäisikö ne ottaa tekstuurista
        let dmrect = Rect::new((winwidth/2) as i32, (gridvars.char_w*gridvars.grid_y) as i32, 191, 17);
        sdl_master.canvas.copy(&dm_image, None, dmrect)?;
    }
         */

        //Character color pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        let mut ccselrect = Rect::new((winwidth-272) as i32, (winheight-30) as i32, 16, 4);
        ccselrect.x = ccselrect.x + (16*iface.cc_selector as i32);
        sdl_master.canvas.fill_rect(ccselrect)?;

        //Background color pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        let mut bcselrect = Rect::new((winwidth-272) as i32, (winheight-10) as i32, 16, 4);
        bcselrect.x = bcselrect.x + (16*iface.bc_selector as i32);
        sdl_master.canvas.fill_rect(bcselrect)?;

        //Charbox pointer
        sdl_master.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); //Red pointer
        let mut cbselrect = Rect::new(0, (gridvars.char_w*((gridvars.grid_y+1) as i16)) as i32, 16, 4);
        cbselrect.x = cbselrect.x + (16*iface.box_selector as i32);
        sdl_master.canvas.fill_rect(cbselrect)?;
        
        draw_grid(&mut gridvars, &mut gridvec_obj, &mut sdl_master);
        //change_gridunit_texture(&mut gridvec_obj, &mut sdl_master, 2, 2, Color::RGBA(255, 0, 0, 255));

        if iface.program_mode != 2 { //Drawing cursor is not rendered when selecting a character

        //Rendering the cursor
        //Upperline
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, (iface.cursor_x+1)*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, curcol1);
        //Left line
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, iface.cursor_x*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, curcol1);
        //Right line
        sdl_master.canvas.line((iface.cursor_x+1)*gridvars.char_w as i16, iface.cursor_y*gridvars.char_h as i16, (iface.cursor_x+1)*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, curcol1);
        //Bottomline
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, (iface.cursor_x+1)*gridvars.char_w as i16, (iface.cursor_y+1)*gridvars.char_h as i16, curcol1);

        //Inner cursor rectangle
        //Upperline
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w+2 as i16, iface.cursor_y*gridvars.char_h+1 as i16, (iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y*gridvars.char_h)+1 as i16, curcol2);
        //Left line
        sdl_master.canvas.line(iface.cursor_x*gridvars.char_w+1 as i16, iface.cursor_y*gridvars.char_h+1 as i16, iface.cursor_x*gridvars.char_w+1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, curcol2);
        //Right line
        sdl_master.canvas.line((iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y*gridvars.char_h)+1 as i16, (iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, curcol2);
        //Bottomline
            sdl_master.canvas.line(iface.cursor_x*gridvars.char_w+1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, (iface.cursor_x+1)*gridvars.char_w-1 as i16, (iface.cursor_y+1)*gridvars.char_h-1 as i16, curcol2);

        }

        if iface.message != "" {
            message(&mut sdl_master, &iface.message);
            iface.message = "".to_string();
        }
    
        if iface.program_mode == 2 {  //2 = Character selection screen
            //Character selection screen
            let x_point = (((winwidth as i16) - iface.charscreenx*gridvars.char_w+1) / 2) as i32;
            let y_point = 80i32;
            let mut selectbgrect = Rect::new(x_point, y_point,
                                             (iface.charscreenx*gridvars.char_w) as u32, ((iface.charscreeny+1)*gridvars.char_h) as u32);

            let mut ansirect = Rect::new(x_point, y_point,
                                         gridvars.char_w as u32, gridvars.char_h as u32);

            sdl_master.canvas.set_draw_color(Color::RGBA(117, 117, 117, 255)); //Gray char select rectangle
            sdl_master.canvas.fill_rect(selectbgrect)?;
            
            for i in 33..255 { //Skipping control characters

                sdl_master.canvas.copy(&iface.ansi_char_vec[i as usize], None, ansirect);

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
                                   (y_point as i16)+(iface.charselector_y as i16)*(gridvars.char_h+1), curcol2);
            //Left line
            sdl_master.canvas.line((x_point as i16)+(iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(iface.charselector_y as i16)*(gridvars.char_h+1),
                                   (x_point as i16)+(iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1), curcol2);
            //Right line
            sdl_master.canvas.line((x_point as i16)+(1+iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(iface.charselector_y as i16)*(gridvars.char_h+1),
                                   (x_point as i16)+(1+iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1), curcol2);
            //Bottom line
            sdl_master.canvas.line((x_point as i16)+(iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1),
                                   (x_point as i16)+(1+iface.charselector_x as i16)*gridvars.char_w,
                                   (y_point as i16)+(1+iface.charselector_y as i16)*(gridvars.char_h+1), curcol2);

        }
    }

    sdl_master.canvas.present();
        
    Ok(())
}

//Writes a message to the control panel for the user. Called from render_screen()
#[cfg(feature = "unsafe_textures")]
fn message(mut sdl_master: &mut SDLMasterVars, msg: &String) -> Result<(), String> {

    let tempwin = sdl_master.canvas.window_mut();
    let size = tempwin.size();
    let winwidth = size.0;
    let winheight = size.1;

    let msg_rect = Rect::new(0 as i32, (winheight-12) as i32, (msg.len()*12) as u32, 16);

    let msgsurface = sdl_master.font.render(msg as &str).blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string())?;
    let msgtexture = sdl_master.texture_creator.create_texture_from_surface(msgsurface).map_err(|e| e.to_string())?;
    sdl_master.canvas.copy(&msgtexture, None, msg_rect);
    //Pitäisi myös tuhota tuo viesti sitten, DESTROY

    Ok(())
}


//This function contains some further init (continuing from main()) and then proceeds to the main loop
#[cfg(feature = "unsafe_textures")]
fn master_function(font_path: &Path, mut sdl_master: &mut SDLMasterVars, mut gridvec_obj: &mut Gridvec, mut gridvars: &mut GridMasterVars, mut iface: &mut Interface) -> Result<(), String> {

    //More init procedures
    init_ansi_textures(&mut iface, &mut sdl_master);
    
    update_panelbox(sdl_master, &mut iface, 1);
    update_panelbox(sdl_master, &mut iface, 2);
    update_panelbox(sdl_master, &mut iface, 3);
    update_panelbox(sdl_master, &mut iface, 4);
    update_panelbox(sdl_master, &mut iface, 5);

    reset_grid(&mut gridvec_obj);

    render_screen(font_path, &mut sdl_master, &mut gridvec_obj, &mut gridvars, &mut iface)?;  //run on tässä koodissa määritelty funktio

    let mut actioncode = String::from("");

    'mainloop: loop {
                    let wait_time = std::time::Duration::from_millis(10);
            std::thread::sleep(wait_time);

        for event in sdl_master.sdl_context.event_pump()?.poll_iter() {
            match event {

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    /*
                    if keycode == Keycode::Escape {
                        break 'mainloop
                    }
                     */

                    if iface.program_mode == 1 {
                        if keycode == Keycode::Right {
                            //TÄHÄN MIELUUMMIN: OLLAANKO GRIDVEKTORISTA ASTUMASSA ULOS
                            //TAI JOS TUO gridvars-tarkistus on tehokkaampi, voisihan siihenkin laittaa siten, että se ei mene ulos gridvec-indeksistä
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
                        else if keycode == Keycode::Num1 {if let Some(var) = iface.keys.get(&String::from("1")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num2 {if let Some(var) = iface.keys.get(&String::from("2")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num3 {if let Some(var) = iface.keys.get(&String::from("3")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num4 {if let Some(var) = iface.keys.get(&String::from("4")) {actioncode=var.to_string();}}
                        else if keycode == Keycode::Num5 {if let Some(var) = iface.keys.get(&String::from("5")) {actioncode=var.to_string();}}

                        if actioncode == "Paint1" { //Painting depending on drawmode
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor1 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 1);
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor1 = iface.bc_selector;
                                update_panelbox(sdl_master, &mut iface, 1);
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1);
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor1);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1);
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor1);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1);
                            }
                        }

                        else if actioncode == "Paint2" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor2 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 2);
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor2 = iface.bc_selector;
                                update_panelbox(sdl_master, &mut iface, 2);
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2);
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor2);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2);
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor2);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2);
                            }
                        }

                        else if actioncode == "Paint3" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor3 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 3);
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor3 = iface.bc_selector;
                                update_panelbox(sdl_master, &mut iface, 3);
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3);
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor3);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3);
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor3);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3);
                            }
                        }

                        else if actioncode == "Paint4" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor4 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 4);
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor4 = iface.bc_selector;
                                update_panelbox(sdl_master, &mut iface, 4);
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4);
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor4);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4);
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor4);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4);
                            }
                        }

                        else if actioncode == "Paint5" {
                            if iface.shift == 1 && iface.previous_com == "SelectCharColor" {
                                iface.fcolor5 = iface.cc_selector;
                                update_panelbox(sdl_master, &mut iface, 5);
                            }
                            else if iface.shift == 1 && iface.previous_com == "SelectBackgroundColor" {
                                iface.bcolor5 = iface.bc_selector;
                                update_panelbox(sdl_master, &mut iface, 5);
                            }
                            else if iface.dm_selector == 1 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5);
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor5);
                            }
                            else if iface.dm_selector == 2 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5);
                            }
                            else if iface.dm_selector == 3 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                                change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                            }
                            else if iface.dm_selector == 4 {
                                gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].backcol = Some(iface.bcolor5);
                            }
                            else if iface.dm_selector == 5 {
                                change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5);
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
                            update_panelbox(sdl_master, &mut iface, 1);
                        }
                        else if actioncode == "CharColorToBox2" {
                            iface.fcolor2 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 2);
                        }
                        else if actioncode == "CharColorToBox3" {
                            iface.fcolor3 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 3);
                        }
                        else if actioncode == "CharColorToBox4" {
                            iface.fcolor4 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 4);
                        }
                        else if actioncode == "CharColorToBox5" {
                            iface.fcolor5 = iface.cc_selector;
                            update_panelbox(sdl_master, &mut iface, 5);
                        }

                        else if actioncode == "BackgroundColorToBox1" { //Directly sets background color pointed by a background color cursor to box 1
                            iface.bcolor1 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 1);
                        }
                        else if actioncode == "BackgroundColorToBox2" {
                            iface.bcolor2 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 2);
                        }
                        else if actioncode == "BackgroundColorToBox3" {
                            iface.bcolor3 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 3);
                        }
                        else if actioncode == "BackgroundColorToBox4" {
                            iface.bcolor4 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 4);
                        }
                        else if actioncode == "BackgroundColorToBox5" {
                            iface.bcolor5 = iface.bc_selector;
                            update_panelbox(sdl_master, &mut iface, 5);
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
                            iface.message = "Select character by using arrow keys and space, esc to cancel".to_string();
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
                        
                        else if actioncode == "PaintChar1" {
                            //println!("kursorin koordinaatit: {}, {}", iface.cursor_x, iface.cursor_y);
                            //println!("gridin koordinaatit: {}, {}", gridvars.grid_x, gridvars.grid_y);
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1);
                        }
                        else if actioncode == "PaintChar2" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2);
                        }
                        else if actioncode == "PaintChar3" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3);
                        }
                        else if actioncode == "PaintChar4" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4);
                        }
                        else if actioncode == "PaintChar5" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5);
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
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor1);
                        }
                        else if actioncode == "ChangeFC2" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor2);
                        }
                        else if actioncode == "ChangeFC3" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor3);
                        }
                        else if actioncode == "ChangeFC4" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor4);
                        }
                        else if actioncode == "ChangeFC5" {
                            change_gridunit_texture(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize, iface.fcolor5);
                        }

                        else if actioncode == "PaintCharNC1" {
                            //Changing character without changing its color
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char1.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                        }
                        else if actioncode == "PaintCharNC2" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char2.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                        }
                        else if actioncode == "PaintCharNC3" {
                            //Changing character without changing its color
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char3.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                        }
                        else if actioncode == "PaintCharNC4" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char4.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
                        }
                        else if actioncode == "PaintCharNC5" {
                            gridvec_obj.gridvector[iface.cursor_x as usize][iface.cursor_y as usize].charstring = iface.char5.clone();
                            change_gridunit_char(gridvec_obj, sdl_master, iface.cursor_x as usize, iface.cursor_y as usize);
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
                                    update_panelbox(sdl_master, &mut iface, 1);
                                }
                                else if iface.box_selector == 2 {
                                    iface.char2 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 2);
                                }
                                else if iface.box_selector == 3 {
                                    iface.char3 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 3);
                                }
                                else if iface.box_selector == 4 {
                                    iface.char4 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 4);
                                }
                                else if iface.box_selector == 5 {
                                    iface.char5 = selchar.to_string();
                                    update_panelbox(sdl_master, &mut iface, 5);
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
                    render_screen(font_path, &mut sdl_master, &mut gridvec_obj, &mut gridvars, &mut iface)?;
                }
                /*
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    game.toggle_state();
                },
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} |
*/

                Event::Quit {..} => break 'mainloop,
                _ => {}
            }
        }
    }
    Ok(())
}


#[cfg(feature = "unsafe_textures")]
fn update_panelbox(sdl_master: &SDLMasterVars, mut iface: &mut Interface, boxnum: u8) -> Result<(), String> {

    if boxnum == 1 {
        let boxslice: &str = &iface.char1;

        if let Some(color) = sdl_master.colortable.get(&iface.fcolor1) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
        //TÄHÄN DESTROY!!!
            iface.panelbox1 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 2 {
        let boxslice: &str = &iface.char2;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor2) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
        //TÄHÄN DESTROY!!!
            iface.panelbox2 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 3 {
        let boxslice: &str = &iface.char3;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor3) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
        //TÄHÄN DESTROY!!!
            iface.panelbox3 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 4 {
        let boxslice: &str = &iface.char4;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor4) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
        //TÄHÄN DESTROY!!!
            iface.panelbox4 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }
    else if boxnum == 5 {
        let boxslice: &str = &iface.char5;
        if let Some(color) = sdl_master.colortable.get(&iface.fcolor5) {
            let boxsurface = sdl_master.font.render(boxslice).blended(color.clone()).map_err(|e| e.to_string())?;
        //TÄHÄN DESTROY!!!
            iface.panelbox5 = Some(sdl_master.texture_creator.create_texture_from_surface(boxsurface).map_err(|e| e.to_string())? );
        }
    }

    Ok(())
}


/*
impl NoRef for Color {
    fn no_ref(&self) -> Color {
        
    }
}
*/


//#[derive(Clone)]
//struct BackColor(u16);

//Struct for each char on the grid.
//#[derive(Clone)]  //#[derive()] laittaa automaattisesti annetun traitin implementaation
#[cfg(feature = "unsafe_textures")]
struct Gridunit {
    charstring: String, //No Option<> is needed since String can be empty.
    //Charstring on string, vaikka se pitää konvertoida &str:ksi. Structin on hyvä omistaa muuttujansa 
    forecol: u8, //Vaihtoehtoisesti olisi voinut käyttää tässä u8-arvoa, jonka konvertoi sdl-coloriksi hashtablella tms.
    //Forecol ei ole option, koska sillä ei ole mitään default-arvoa rendattaessa
    backcol: Option<u8>,
//    chartexture: sdl2::render::Texture, //Ilman Optionia
    chartexture: Option<sdl2::render::Texture>, //Jos texture on None, sitten ei piirretä mitään merkkiä
//    chartexture: Option<&'a sdl2::render::Texture>,
    //Ilmeisesti pitäisi luoda täällä se tekstuuri: https://rust-sdl2.github.io/rust-sdl2/sdl2_sys/fn.SDL_CreateTexture.html
    //chartexture: Option<*mut sdl2_sys::SDL_Texture>,
}


/*
impl Gridunit {
    unsafe fn free_texture(mut gridvec_obj: &mut Gridvec, x: usize, y: usize) {
        if let Some(txtr) = &gridvec_obj.gridvector[x][y].chartexture {
            txtr.destroy();
        }
    }
}
*/

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
        //Tässä pitäisi kysyä onko siellä jotain?
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
        //Tässä pitäisi kysyä onko siellä jotain?
        gridarg.gridvector[x][y].chartexture = Some(arg1.texture_creator.create_texture_from_surface(charsurface).map_err(|e| e.to_string())? );
    }
    Ok(())
}


//A function which fills ansi_char_vec with textures of all ansi characters
#[cfg(feature = "unsafe_textures")]
fn init_ansi_textures(mut iface: &mut Interface, sdl_arg: &mut SDLMasterVars) -> Result<(), String> {
    for i in 0..255 {
        //if i == 0 || i == 32 || i == 255 {continue;}
        if let Some(getchar) = iface.chartable.get(&i) {
            //println!("{}", getchar);
            let ansichar: &str = &getchar;
            let ansisurface = sdl_arg.font.render(ansichar).blended(Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string())?;
            let ansitexture = sdl_arg.texture_creator.create_texture_from_surface(ansisurface).map_err(|e| e.to_string())?;
            iface.ansi_char_vec.push(ansitexture);
            //These textures will live as long as the program is running so they will not be explicitely destroyed
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
    //Tässä olisi parempi tapa tehdä looppi, mutta columnin pitäisi olla mutable
    /*
    for row in grid_arg2.gridvector.iter() {
    for column in row.iter() {
    column.charstring = 0;
    //(*column).forecol = 0;
    //(*column).backcol = 0;
}
}
     */
    //self
}

/*
fn get_forecolor(cnum: u8) -> sdl2::pixels::Color {
    if cnum = 0 {return Color::RGBA(0, 0, 0, 255);}
    if cnum = 1 {return Color::RGBA(10, 10, 10, 255);}  //MUUTA
    //Tähän joku cerror, jos on väärä argumentti?
    //16 väriä tähän...
}

fn get_backcolor(cnum: u8) -> sdl2::pixels::Color {
    if cnum = 0 {return Color::RGBA(0, 0, 0, 255);}
    if cnum = 1 {return Color::RGBA(10, 10, 10, 255);}  //MUUTA
    //Tähän joku cerror, jos on väärä argumentti?
}
*/
#[cfg(feature = "unsafe_textures")]
struct GridMasterVars {
    //Grid size
    grid_x: u16, //These values are one-based whereas gridvector and cursor values from interface struct are zero-based
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

/*
fn sdl_init_function(arg: &mut SDLMasterVars) {
//    arg.sdl_context = Some(sdl2::init());
//    arg.video_subsys = Some(arg.sdl_context.unwrap().video());
}
*/

//Renders grid by going through gridunits of the gridvector and drawing background color tiles and characters (when found)
//Character rendering is done elsewhere, it needs not to be done all the time.
#[cfg(feature = "unsafe_textures")]
fn draw_grid(arg1: &mut GridMasterVars, arg2: &mut Gridvec, arg3: &mut SDLMasterVars) {  //ILMEISESTI tähän ei tule structin lifetimejä, kun niitä ei käytetä

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

            arg3.canvas.fill_rect(unitrect);

            //If the grid unit has a texture, it will be rendered
            if let Some(ctext) = arg2.gridvector[counter1][counter2].chartexture.as_ref() {
                arg3.canvas.copy(ctext, None, Some(unitrect)); //Mikä toinen parametri on?
            }
            
            counter2 += 1;
        }
        counter2 = 0;
        counter1 += 1;
    }
}



#[cfg(feature = "unsafe_textures")]
fn main() -> Result<(), String> {

    let args: Vec<_> = env::args().collect();
    //env-moduulin args (env::args() on iteraattori prosessin (ilmeisesti komentorivi-)argumenttien yli, palauttaa kustakin stringin
    //Tässä luodaan siis vektori iteroimalla, jolloin käytetään collect():ia. Tässä tapauksessa kerätään prosessin argumentit.
    //tuossa Vec<_> on yleensä datatyyppi alaviivan tilalla, mutta ilmeisesti sitä ei tarvita

    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]");
        std::process::exit(1)
    }

    let path: &Path = Path::new(&args[1]);

    let default_grid_w = 50u16; //50 tässä antaa gridveciin ja interfaceen rangen 0-49 eli tämä arvo on one-based
    let default_grid_h = 20u16; //Sama kuin yllä
    let default_char_w = 16i16;
    let default_char_h = 16i16;

        //Path on tiedostopolkua käsittelevä tyyppi, jossa on sen käsittelyä helpottavia operaatioita. Tyyppi on unsized, joten pitää käyttää pointteria, kuten & tai Box
        //new luo Pathin string slicestä. 

    //Tässä siis SDLMasterVars -structin muuttujat luodaan ensin ja sitten vasta laitetaan sinne itse structiin, kun structin instanssi vihdoin luodaan. Jos yritti ensin luoda instanssin, piti käyttää Option<T> -muuttujia, jotka sotkee ohjelman toiminnan niin, ettei alustettuihin muuttujiin kuuluvia metodeita (esim. sdl_context.video()) sitten löydykään. Ehkä myös jotkut kääntäjän ehdottamat phantom variablet olisi voinut laittaa tuohon structin muuttujien datatyypeiksi optionin sijaan, mutta jäi silti kuva, että vain structin ulkopuolella alustaen nuo muuttujat voivat toimia. 
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
//    let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    //+100 is the space for interface's panel
    let window = video_subsys
        .window("LammasDRAW", (default_grid_w*(default_char_w as u16)) as u32, (default_grid_h*(default_char_h as u16)+50) as u32)
        //.resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font(path, 64)?; //Toinen argumentti on fonttikoko
    
    //    let mut SDL_object = SDLMasterVars {sdl_context, video_subsys, ttf_context, window, canvas, texture_creator, font};
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

    interfaceobj.keys.insert(String::from("Z"), String::from("Paint1"));
    interfaceobj.keys.insert(String::from("X"), String::from("Paint2"));
    interfaceobj.keys.insert(String::from("C"), String::from("Paint3"));
    interfaceobj.keys.insert(String::from("V"), String::from("Paint4"));
    interfaceobj.keys.insert(String::from("B"), String::from("Paint5"));

    interfaceobj.keys.insert(String::from("A"), String::from("DrawmodeLeft"));
    interfaceobj.keys.insert(String::from("S"), String::from("DrawmodeRight"));
    interfaceobj.keys.insert(String::from("E"), String::from("CcolorSelectorLeft"));
    interfaceobj.keys.insert(String::from("R"), String::from("CcolorSelectorRight"));
    interfaceobj.keys.insert(String::from("D"), String::from("BcolorSelectorLeft"));
    interfaceobj.keys.insert(String::from("F"), String::from("BcolorSelectorRight"));
    interfaceobj.keys.insert(String::from("T"), String::from("SelectCharColor"));
    interfaceobj.keys.insert(String::from("G"), String::from("SelectBackgroundColor"));
    interfaceobj.keys.insert(String::from("Q"), String::from("BoxSelectorLeft"));
    interfaceobj.keys.insert(String::from("W"), String::from("BoxSelectorRight"));
    interfaceobj.keys.insert(String::from("1"), String::from("SelectChar"));


    //NOTE: The current font has no characters 0-32 and they are not used. If they are taken into use, this will have some implications for the code, especially for some loops plus controlling the character selection cursor
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

    /*
    interfaceobj.keys.insert(String::from("Z"), String::from("PaintCharNC1"));
    interfaceobj.keys.insert(String::from("X"), String::from("PaintCharNC2"));
    interfaceobj.keys.insert(String::from("C"), String::from("PaintCharNC3"));
    interfaceobj.keys.insert(String::from("V"), String::from("PaintCharNC4"));
    interfaceobj.keys.insert(String::from("B"), String::from("PaintCharNC5"));
    
    interfaceobj.keys.insert(String::from("A"), String::from("PaintChar1"));
    interfaceobj.keys.insert(String::from("S"), String::from("PaintChar2"));
    interfaceobj.keys.insert(String::from("D"), String::from("PaintChar3"));
    interfaceobj.keys.insert(String::from("F"), String::from("PaintChar4"));
    interfaceobj.keys.insert(String::from("G"), String::from("PaintChar5"));

    interfaceobj.keys.insert(String::from("Q"), String::from("ChangeFC1"));
    interfaceobj.keys.insert(String::from("W"), String::from("ChangeFC2"));
    interfaceobj.keys.insert(String::from("E"), String::from("ChangeFC3"));
    interfaceobj.keys.insert(String::from("R"), String::from("ChangeFC4"));
    interfaceobj.keys.insert(String::from("T"), String::from("ChangeFC5"));

    interfaceobj.keys.insert(String::from("1"), String::from("ChangeBC1"));
    interfaceobj.keys.insert(String::from("2"), String::from("ChangeBC2"));
    interfaceobj.keys.insert(String::from("3"), String::from("ChangeBC3"));
    interfaceobj.keys.insert(String::from("4"), String::from("ChangeBC4"));
    interfaceobj.keys.insert(String::from("5"), String::from("ChangeBC5"));
    */

/*    let dm_image = texture_creator.load_texture(/home/bergfink/rust/editor2/drawmode.png)?;
    interfaceobj.drawmode_img = Some(dm_image);
*/

    
//    let mut SDL_object = SDLMasterVars {sdl_context: None, video_subsys: None};
//    sdl_init_function(&mut sdl_masterobj);

    //Tätä ei ilmeisesti tarvittukaan
    //let mut generic_gridunit = Gridunit {charstring: " ".to_string(), forecol: Color::RGBA(255, 255, 255, 255), backcol: None, chartexture: None};

    //Default values for the grid
    /*
    let mut the_grid = Grid {
        grid_x: 50, grid_y: 50, gridvector: [[Gridunit; grid_x]; grid_y]
    };
    */

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

    //luo vektori ja lisää geneerisiä yksiköitä ilman clonea
    //let mut v = Vec::new(); for _ in 0..20 { v.push(Struct {}); }

    //  UUSI EHDOTUS  gridvector: vec![vec![1,2,3],vec![4,5,6]];
    //HETKINEN!! tarvitseeko vektorin sittenkään olla resizable vai voiko sen tehdä aina uudestaan vanhan pohjalta?
    //tässä oli ei-dynaaminen esimerkki https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d1fddc4253bfd15685225bb57e9aed9b
    
    //Jos funktio palauttaa arvon, paluutyyppi annetaan nuolen jälkeen. Resultin tehtävä on hoitaa virheet paluutyypin yhteydessä. Resultin OK-puoli (vasen) on jätetty tyhjäksi, ja virheen sattuessa (oikea puoli) palautetaan String -tyyppinen muuttuja (missäkÃ¶hän se määritellään)

    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());  //TTF:n funktio joka palauttaa version
    //Huutomerkki (println!) tarkoittaa, että kyseessä on makro ja se ajetaan kääntÃ¶ajassa. {} on paikka, mihin pilkun jälkeen annetut arvot tulee
    //Miksi tässä on näitä makroja?

    master_function(path, &mut sdl_masterobj, &mut gridvector_obj, &mut gridvars, &mut interfaceobj)?;  //run on tässä koodissa määritelty funktio

    Ok(()) //Tämä on ilmeisesti std::resultista ja antaa resultin tjsp.
}

#[cfg(not(feature = "unsafe_textures"))]
pub fn main(){}

/*
lifetime syntax
struct Gridunit<'a> {
    chartexture: Option<&'a sdl2::render::Texture<'a>>, //Jos texture on None, sitten ei piirretä mitään merkkiä
struct SDLMasterVars<'a, 'b> {
    font: sdl2::ttf::Font<'a, 'b>,
struct Gridvec<'a> {
    gridvector: Vec<Vec<Gridunit<'a>>>,  //A two dimensional vector, x & y

*/

/*
            //Rect X = (window's width - char selection box width) / 2
            let mut rowlength = 20i16;
            let mut selectbgrect = Rect::new((((winwidth as i16) - rowlength*gridvars.char_w) / 2) as i32, 80,
                                             (rowlength*gridvars.char_w) as u32,
                                             (((222./(rowlength as f32)).ceil() as i16)*(gridvars.char_h+1)) as u32);
            //222. (255-33) refers to the number of characters without control chars. The dot means it's a float
            
            let mut ansirect = Rect::new((((winwidth as i16) - rowlength*gridvars.char_w) / 2) as i32, 80,
                                         gridvars.char_w as u32, gridvars.char_h as u32);
            let chars_in_row = winwidth/((gridvars.char_w+1) as u32)*(gridvars.char_w as u32); //Pitääkö tästä miinustaa vielä yksi?

            sdl_master.canvas.set_draw_color(Color::RGBA(117, 117, 117, 255)); //Gray
            sdl_master.canvas.fill_rect(selectbgrect)?;
            
            for i in 33..255 { //Skipping control characters

                sdl_master.canvas.copy(&iface.ansi_char_vec[i as usize], None, ansirect);

                if (ansirect.x as u32 + (2*gridvars.char_w as u32)) > (((((winwidth as i16) - rowlength*gridvars.char_w) / 2) as u32) + (rowlength*gridvars.char_w) as u32)  {
                    ansirect.y = ansirect.y+(gridvars.char_h as i32)+1;
                    ansirect.x = (((winwidth as i16) - rowlength*gridvars.char_w) / 2) as i32;
                }
                else {ansirect.x = ansirect.x + (gridvars.char_w as i32);}
            }
*/
