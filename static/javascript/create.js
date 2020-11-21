const optionsDiv = document.querySelector(".options-all");
var count;

for (let i = 0; i < 2; i++) {
	let form = document.createElement("div");

	count = i + 1;

	form.innerHTML = `
	<input class="form-control" placeholder="Enter a option" onfocus="addOption(this)" onblur="removeOption(this)">
	<br>
	`

	optionsDiv.appendChild(form);
}

function addOption(element) {
	let form = document.createElement("div");

	count = count + 1;

	form.innerHTML = `
	<input class="form-control" placeholder="Enter a option" onfocus="addOption(this)" onblur="removeOption(this)">
	<br>
	`
	
	if (element.parentNode.parentNode.lastElementChild === element.parentNode && element.parentNode.parentNode.firstChild.value !== "") {
		optionsDiv.appendChild(form);
	}
}

function removeOption(element) {
	if (element.value == "" && element.parentNode.parentNode.childNodes.length > 2) {
		element.parentNode.remove();
	}
}

const submitPoll = document.querySelector(".submit");

submitPoll.addEventListener("click", (event) => {
	event.preventDefault();

	var values = [];

	const ipCheck = document.querySelector("#ip-check").checked;
	const description = document.querySelector(".description").value;
	const question = document.querySelector(".question").value;
	
	optionsDiv.childNodes.forEach(element => {
		let val = element.children[0].value.trim();

		if (val !== "") {
			values.push(val);
		}
	})

	const data = {
		"question": question,
		"description": description,
		"options": values,
		"settings": {
			"ip-check": ipCheck
		}
	}

	var xhr = new XMLHttpRequest();

	xhr.open("POST", "/api/poll/", true);
	xhr.setRequestHeader('Content-Type', 'application/json');

	xhr.send(JSON.stringify(data));

	xhr.onload = () => {
		window.location.replace("/vote/" + JSON.parse(xhr.responseText).id);
	}
});
