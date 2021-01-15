import * as wasm from "wasm-app";
import { Input } from "wasm-app";

// Input


function read_input() {
	const inmune_init = parseInt(document.getElementById("inmune").value);		
	const concert_hall = document.getElementById("concert_hall").checked;
	const bakery = document.getElementById("bakery").checked;
	const school = document.getElementById("school").checked;
	const pharmacy = document.getElementById("pharmacy").checked;
	const restaurant = document.getElementById("restaurant").checked;
	const gym = document.getElementById("gym").checked;
	const supermarket = document.getElementById("supermarket").checked;
	const shopping_center = document.getElementById("shopping_center").checked;
	return Input.new(
		inmune_init, 
		concert_hall, 
		bakery,
		school,
		pharmacy,
		restaurant,
		gym,
		supermarket,
		shopping_center,
	);
}

// Interactive elements
const pre = document.getElementById("output");
const simulateButton = document.getElementById("simulate");
const simulate100Button = document.getElementById("simulate_100");
// const simulate200Button = document.getElementById("simulate_200");
// const simulate1000Button = document.getElementById("simulate_1000");

// Interactions
simulateButton.addEventListener("click", event => {
	pre.textContent = "Computing!";
	setTimeout(function() {
		console.time("simulation_js");
		const output = read_input().message_js();
		pre.textContent = output;
		console.timeEnd("simulation_js");
	}, 1);
});

simulate100Button.addEventListener("click", event => {
	pre.textContent = "Computing!";
	setTimeout(function() {
		console.time("simulation_many_js");
		const output = read_input().message_many_js(100);
		pre.textContent = output;
		console.timeEnd("simulation_many_js");		
	}, 1);
});

// simulate200Button.addEventListener("click", event => {
// 	pre.textContent = "Computing!";
// 	setTimeout(function() {
// 		console.time("simulation_many_js");
// 		const output = read_input().message_many_js(200);
// 		pre.textContent = output;
// 		console.timeEnd("simulation_many_js");		
// 	}, 1);
// });

// simulate1000Button.addEventListener("click", event => {
// 	pre.textContent = "Computing!";
// 	setTimeout(function() {
// 		console.time("simulation_many_js");
// 		const output = read_input().message_many_js(1000);
// 		pre.textContent = output;
// 		console.timeEnd("simulation_many_js");		
// 	}, 1);
// });