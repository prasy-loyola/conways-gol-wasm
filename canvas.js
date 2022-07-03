function draw_rect(context,x,y,w,h,r,g,b,a){

    let style = "#";
    style += Number(r).toString(16).padStart(2,"0");
    style += Number(g).toString(16).padStart(2,"0");
    style += Number(b).toString(16).padStart(2,"0");
    style += Number(a).toString(16).padStart(2,"0");

    console.log(x,y,w,h,r,g,b,a,style)
    context.fillStyle = style;
    context.fillRect(x,y,w,h);
}

export {draw_rect}