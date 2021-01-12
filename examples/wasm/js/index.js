import("../pkg/index.js").then(module => {
	function getInputs() { // -> (usize, bool, ..., bool)
		var inmune_init = parseInt(document.getElementById("inmune").value);		

		var concert_hall = document.getElementById("concert_hall").checked;
		var bakery = document.getElementById("bakery").checked;
		var school = document.getElementById("school").checked;
		var pharmacy = document.getElementById("pharmacy").checked;
		var restaurant = document.getElementById("restaurant").checked;
		var gym = document.getElementById("gym").checked;
		var supermarket = document.getElementById("supermarket").checked;
		var shopping_center = document.getElementById("shopping_center").checked;

		var buildings = [concert_hall, bakery, school, pharmacy, restaurant, gym, supermarket, shopping_center];

		var inputs = [inmune_init, buildings]

		return inputs;
	}

	function simulation() {
		var inputs = getInputs();
		document.getElementById("simulation_result").innerHTML = inputs;
	}

	function simulations_1000() {
		var inputs = getInputs();
		document.getElementById("simulations_1000_result").innerHTML = inputs;
	}	

	module.alert("here!");
}).catch(console.error);

function a() {
	document.getElementById("b").innerHTML = "a";
}

