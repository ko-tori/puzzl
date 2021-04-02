var mouseDown = false;
var mousePageX = 0;
var mousePageY = 0;
var mouseGridX = 0;
var mouseGridY = 0;
var windowWidth = 0;
var windowHeight = 0;
var bounds;
var centerCoord = [0, 0];
var pixelsPerCoord = 25;

var canvas;
var ctx;

function update(dt) {
	let minX = (-windowWidth / 2) / pixelsPerCoord + centerCoord[0];
	let maxX = (windowWidth / 2) / pixelsPerCoord + centerCoord[0];
	let minY = (-windowHeight / 2) / pixelsPerCoord + centerCoord[1];
	let maxY = (windowHeight / 2) / pixelsPerCoord + centerCoord[1];
	bounds = [minX, maxX, minY, maxY];
	$('#debug').text(`(${mouseGridX.toFixed(2)}, ${mouseGridY.toFixed(2)})`);
}

function render() {
	ctx.clearRect(0, 0, windowWidth, windowHeight);

	ctx.save();
	ctx.translate(windowWidth / 2, windowHeight / 2);
	ctx.scale(pixelsPerCoord, pixelsPerCoord);
	ctx.translate(-centerCoord[0], -centerCoord[1]);

	ctx.fillRect(-1, -1, 2, 2);

	ctx.restore();
}

let prev;
function frame(timestamp) {
	let dt;
	if (!prev) {
		prev = timestamp;
		requestAnimationFrame(frame);
		return;
	} else {
		dt = timestamp - prev;
	}
	prev = timestamp;

	update(dt);
	render();

	requestAnimationFrame(frame);
}

$(document).ready(() => {
	resizeWindow();

	canvas = document.getElementById('canvas');
	ctx = canvas.getContext('2d');

	requestAnimationFrame(frame);
});

function resizeWindow() {
	windowWidth = $(window).width();
	windowHeight = $(window).height();
	$('canvas').attr('width', windowWidth);
	$('canvas').attr('height', windowHeight);
}

window.addEventListener('resize', resizeWindow);

function gridConvert(px, py) {
	return [(px - windowWidth / 2) / pixelsPerCoord + centerCoord[0], 
		(py - windowHeight / 2) / pixelsPerCoord + centerCoord[1]];
}

$(document).contextmenu(e => {
	e.preventDefault();
});

$(document).mousedown(e => {
	mouseDown = true;
	mouseMoved = false;
});

$(document).mouseup(e => {
	mouseDown = false;
});

$(document).mousemove(e => {
	if (mouseDown) {
		mouseMoved = true;
	}
	prevPageX = mousePageX;
	prevPageY = mousePageY;
	prevGridX = mouseGridX;
	prevGridY = mouseGridY;
	mousePageX = e.pageX;
	mousePageY = e.pageY;
	[mouseGridX, mouseGridY] = gridConvert(mousePageX, mousePageY);

	if (mouseDown) {
		centerCoord[0] += (prevPageX - mousePageX) / pixelsPerCoord;
		centerCoord[1] += (prevPageY - mousePageY) / pixelsPerCoord;
	}
});

window.addEventListener('wheel', e => {
	let d = Math.exp((e.deltaY > 0 ? -1 : 1) * 0.1);
	pixelsPerCoord *= d;
	let newloc = gridConvert(mousePageX, mousePageY);
	centerCoord[0] += (mouseGridX - newloc[0]);
	centerCoord[1] += (mouseGridY - newloc[1]);
});