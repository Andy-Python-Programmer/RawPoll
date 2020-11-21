let id = window.location.pathname.split("/");

id = id[id.length - 1]

const opts = document.querySelector(".opts");
const title = document.querySelector(".title");
const description = document.querySelector(".description");

fetch("/api/poll/" + id)
	.then(
		response => {
				response.json().then(data => {
						title.innerHTML = data.question;
						description.innerHTML = "<strong>" + data.description + "</strong>";

						Object.entries(data.options).forEach(element => {
								const [key, value] = element;
								
								opts.innerHTML += `
								<input type="radio" name="choice" value="${key}" style="margin-right: 5px" >${key}
								<br>
								`
						})
				});
		}
);

const submitOpt = document.querySelector(".submit");
const resultsButton = document.querySelector(".results");

resultsButton.addEventListener("click", (event) => {
		event.preventDefault();

		window.location.replace(window.location.pathname.replace("vote", "poll"));
})

submitOpt.addEventListener("click", (event) => {
		event.preventDefault();

		const rbs = document.querySelectorAll('input[name="choice"]');
		let selectedValue;

		for (const rb of rbs) {
				if (rb.checked) {
						selectedValue = rb.value;
						break;
				}
		}
		
		const data = {
				id: id,
				option: selectedValue
		}

		var xhr = new XMLHttpRequest();

		xhr.open("POST", "/api/vote/", true);
		xhr.setRequestHeader('Content-Type', 'application/json');

		xhr.send(JSON.stringify(data));

		xhr.onload = () => {
			const body = document.querySelector("body");
			const json = JSON.parse(xhr.responseText);

			if (json.error != null) {
					body.innerHTML += `
					<div class="alert alert-danger" role="alert">
							<h4 class="alert-heading">Error!</h4>
							<hr>
							${json.error}
					</div>`;
			}
		}
	}
);