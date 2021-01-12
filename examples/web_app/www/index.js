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
const simulateManyButton = document.getElementById("simulate_many");

// Interactions
simulateButton.addEventListener("click", event => {
	console.time("simulation_js");
	const input = read_input();
	pre.textContent = input.message_js();
	console.timeEnd("simulation_js");
});

simulateManyButton.addEventListener("click", event => {
	console.time("simulation_many_js");
	const input = read_input();
	pre.textContent = input.message_many_js();
	console.timeEnd("simulation_many_js");
});