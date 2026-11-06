//! Timing shared by the GPU card artwork and the UI labels.
pub const HOLD_SECONDS: f32 = 4.8;

#[derive(Clone, Copy)]
pub struct Reveal {
    pub bounds: [f32; 4],
    pub front: bool,
    pub alpha: f32,
}

pub fn layout(w: f32, h: f32, age: f32) -> Reveal {
    let age = age.max(0.0);
    let ch = (h - 180.0).clamp(280.0, 520.0);
    let cw = ch * 0.70;
    let k = (age / 0.65).clamp(0.0, 1.0);
    let ease = 1.0 - (1.0-k).powi(3);
    let bounce = if age > 0.57 { ((age-0.57)*14.0).sin() * (-(age-0.57)*7.0).exp() * 0.045 } else { 0.0 };
    let scale = 0.82 + ease*0.18 + bounce;
    let flip = if age < 0.32 { (1.0-age/0.32).max(0.035) } else { ((age-0.32)/0.25).clamp(0.035,1.0) };
    let (dw,dh) = (cw*scale*flip,ch*scale);
    let (cx,cy) = (w*0.5,h*0.5+8.0+(1.0-ease)*38.0);
    Reveal { bounds:[cx-dw*0.5,cy-dh*0.5,cx+dw*0.5,cy+dh*0.5], front:age>=0.32, alpha:(age/0.16).clamp(0.0,1.0) }
}

pub fn accent(stars:u8) -> [f32;4] {
    match stars { 3=>[1.0,0.76,0.30,1.0],2=>[0.44,0.76,1.0,1.0],_=>[0.38,0.93,0.79,1.0] }
}

pub fn visible_stars(age:f32,stars:u8) -> u8 {
    (0..stars).filter(|i|age>=0.82+*i as f32*0.22).count() as u8
}

/// Card frame, rotating light rays and a deterministic burst of sparkles.
pub fn draw_stage(g:&mut crate::gfx::Gfx,w:f32,h:f32,age:f32,stars:u8,fresh:bool) -> Reveal {
    let r=layout(w,h,age);
    let [x0,y0,x1,y1]=r.bounds;
    let (cx,cy)=((x0+x1)*0.5,(y0+y1)*0.5);
    let color=accent(stars);
    g.rect(0.0,0.0,w,h,[0.005,0.015,0.035,0.88]);
    g.radial(cx,cy,(y1-y0)*0.88,48,[color[0],color[1],color[2],0.22*r.alpha],[0.0,0.0,0.0,0.0]);
    for i in 0..12 {
        let a=i as f32/12.0*std::f32::consts::TAU+age*0.12;
        let inner=(y1-y0)*0.23;
        let outer=(y1-y0)*0.95;
        let p=|angle:f32,r:f32|(cx+angle.cos()*r,cy+angle.sin()*r);
        g.tri3(p(a,inner),p(a-0.055,outer),p(a+0.055,outer),
            [color[0],color[1],color[2],0.10*r.alpha],[color[0],color[1],color[2],0.0],[color[0],color[1],color[2],0.0]);
    }
    let burst=(age-0.32).max(0.0);
    for i in 0..40 {
        let seed=crate::rng::hash2(i,7,73);
        let speed=crate::rng::hash2(i,11,73);
        let theta=i as f32*2.39996;
        let life=(1.0-burst/(2.0+seed*2.5)).clamp(0.0,1.0);
        let radius=(30.0+burst*(60.0+speed*95.0))*(if fresh {1.0}else{0.75});
        let (px,py)=(cx+theta.cos()*radius,cy+theta.sin()*radius+burst*burst*12.0);
        let size=(1.5+seed*2.5)*life;
        let c=[color[0],color[1],color[2],life*r.alpha*0.9];
        g.tri((px,py-size*2.0),(px+size,py),(px,py+size*2.0),c);
        g.tri((px,py-size*2.0),(px,py+size*2.0),(px-size,py),c);
    }
    for layer in (1..=5).rev() {
        let pad=layer as f32*2.5;
        g.rect_outline(x0-pad,y0-pad,x1+pad,y1+pad,2.0,
            [color[0],color[1],color[2],(0.12/layer as f32)*r.alpha]);
    }
    g.rect(x0+7.0,y0+10.0,x1+7.0,y1+10.0,[0.0,0.0,0.0,0.5*r.alpha]);
    g.rect_vgrad(x0,y0,x1,y1,[0.07,0.19,0.25,r.alpha],[0.015,0.04,0.08,r.alpha]);
    g.rect_outline(x0,y0,x1,y1,2.5,[color[0],color[1],color[2],r.alpha]);
    if !r.front {
        let radius=((x1-x0)*0.32).min((y1-y0)*0.18);
        g.circle(cx,cy,radius,40,[color[0],color[1],color[2],0.12*r.alpha]);
        g.ring(cx,cy,radius,2.0,40,[color[0],color[1],color[2],r.alpha]);
        // A shell-like spiral, keeping the card back independent of language.
        let mut prev=(cx,cy);
        for i in 1..96 {
            let t=i as f32/95.0;
            let a=t*std::f32::consts::TAU*2.2;
            let p=(cx+a.cos()*radius*t*0.85,cy+a.sin()*radius*t*0.85);
            g.line(prev,p,1.7,[color[0],color[1],color[2],r.alpha]);
            prev=p;
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flip_reveals_front_before_stars_arrive() {
        assert!(!layout(1360.0,720.0,0.15).front);
        assert!(layout(1360.0,720.0,0.5).front);
        assert_eq!(visible_stars(0.7,3),0);
        assert_eq!(visible_stars(1.05,3),2);
        assert_eq!(visible_stars(2.0,3),3);
    }
    #[test]
    fn card_stays_inside_playable_view() {
        for (w,h) in [(1360.0,720.0),(1000.0,470.0)] {
            for age in [0.0,0.16,0.32,0.5,0.75,1.0,4.0] {
                let [x0,y0,x1,y1]=layout(w,h,age).bounds;
                assert!(x0>=0.0 && x1<=w && y0>=40.0 && y1<=h-25.0);
            }
        }
    }
}
