var optionsDiv: Element = document.querySelector(".options-all")!;

var count: number = 0;

for (let i: number = 0; i < 2; i++) {
	let form = document.createElement("div");

	count = i + 1;

	form.innerHTML = `
	<input class="form-control" placeholder="Enter a option" onfocus="addOption(this)" onblur="removeOption(this)">
	<br>
	`

	optionsDiv.appendChild(form);
}

function addOption(element: Element): void {
	let form = document.createElement("div");

	count = count + 1;

	form.innerHTML = `
	<input class="form-control" placeholder="Enter a option" onfocus="addOption(this)" onblur="removeOption(this)">
	<br>
	`
	
	if (element.parentNode!.parentNode!.lastElementChild === element.parentNode && (element.parentNode!.parentNode!.firstChild as HTMLInputElement).value !== "") {
		optionsDiv.appendChild(form);
	}
}

function removeOption(element: Element): void {
	if ((element as HTMLInputElement).value == "" && element.parentNode!.parentNode!.childNodes.length > 2) {
		(element.parentNode as Element).remove();
	}
}

var submitPoll: Element = document.querySelector(".submit")!;

submitPoll.addEventListener("click", (event) => {
	event.preventDefault();

	let values: Array<string> = [];

	const ipCheck = (document.querySelector("#ip-check") as HTMLInputElement).checked;
	const description = (document.querySelector(".description") as HTMLInputElement).value;
	const question = (document.querySelector(".question") as HTMLInputElement).value;
	
	optionsDiv.childNodes.forEach(element => {
		let val = ((element as Element).children[0] as HTMLInputElement).value.trim();

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