var id: string[] | string = window.location.pathname.split("/");

id = id[id.length - 1]

var title = document.querySelector(".title")!;
var description = document.querySelector(".description")!;
var options = document.querySelector(".options")!;

var prevPoll: Object;

fetch("/api/poll/" + id)
	.then(
		response => {
			response.json().then(function(data) {
				title.innerHTML = data.question;
				description.innerHTML = data.description;

				prevPoll = data.options;

				Object.entries(data.options).forEach((opt) => {
					const [key, value] = opt;

					options.innerHTML += `
					<li class="list-group-item d-flex justify-content-between align-items-center">
						${key}
						<span class="badge badge-primary badge-pill">${value}</span>
					</li>
					`;
				})
			}
		)
	});

	function isEquivalent(a: Object, b: Object) {
		// Create arrays of property names
		var aProps = Object.getOwnPropertyNames(a);
		var bProps = Object.getOwnPropertyNames(b);

		// If number of properties is different,
		// objects are not equivalent
		if (aProps.length != bProps.length) {
			return false;
		}

		for (var i = 0; i < aProps.length; i++) {
			var propName = aProps[i];

			// If values of same property are not equal,
			// objects are not equivalent
			if ((a as any)[propName] !== (b as any)[propName]) {
				return false;
			}
		}

		// If we made it this far, objects
		// are considered equivalent
		return true;
	}

setInterval(async () => {
	const response = await fetch("/api/poll/" + id);
	const poll = await response.json();

	if (!isEquivalent(poll.options, prevPoll)) {
		options.innerHTML = "";

		Object.entries(poll.options).forEach((opt) => {
			const [key, value] = opt;

			options.innerHTML += `
			<li class="list-group-item d-flex justify-content-between align-items-center">
				${key}
				<span class="badge badge-primary badge-pill">${value}</span>
			</li>
			`;
		})
	};
}, 500);