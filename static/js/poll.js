const body = document.querySelector("body");
const opts = document.querySelector(".opts");
const id = window.location.href.split("/poll/");

var prevData = {};
var curCharInstance;

function chartShow(data) {
    let pollChart = document.getElementById("poll").getContext('2d');
     
    if (curCharInstance != null) {
        curCharInstance.destroy();
    } 

    let vals = {};

    for (var i = 0; i < data.options.split(",").length; i++) {
        const value = data.options.split(",")[i].split(":");

        vals[value[0].trim()] = value[1].trim()
    };

    pollChart.canvas.parentNode.style.height = window.innerHeight / 2 + "px";
    pollChart.canvas.parentNode.style.width = window.innerHeight / 2 + "px";

    curCharInstance = new Chart(pollChart, {
        type: 'doughnut',
        data: {
            labels: Object.keys(vals),
            datasets: [ {
                data: Object.values(vals),
                backgroundColor: ['#49A9EA', '#36CAAB']
            }]
        },
        options: {
            title: {
                text: "Data",
                display: true
            },
            animation: {
                tension: {
                    easing: 'linear',
                    from: 1,
                    to: 0,
                    loop: true
                }
            },
        }
    });
}

function updateData() {
    fetch(id[0] + "/api/poll/" + id[1])

    .then(response => response.json())
    .then(data => {
        if (prevData !== data.options) {
            prevData = data.options;

            chartShow(data);
        }
    })
}

function main() {
    fetch(id[0] + "/api/poll/" + id[1])
    .then(response => response.json())
    .then(data => {
        const valueLink = document.querySelector(".value");

        valueLink.innerHTML = `
        <input type="text" class="form-control" value="${window.location.href}" placeholder="Some path" id="copy-input" readonly>

        <span class="input-group-btn">
            <button class="btn btn-default" type="button" id="copy-button"
                data-toggle="tooltip" data-placement="button"
                title="Copy to Clipboard">
              Copy
            </button>
        </span>
        `

        opts.innerHTML += `
        <h1>Title: ${data.title}</h1>
        <h1>Description: ${data.description}</h1>
        `
        updateData();

        for (var i = 0; i < data.options.split(",").length; i++) {
            const value = data.options.split(",")[i].split(":");
    
            opts.innerHTML += `
            <button class="btn btn-primary" onClick="update(this)" id="${value[0].trim()}">${value[0].trim()}</button>
            `
        };
    });
}

main();

function update(element) {
    fetch(id[0] + "/api/poll/" + id[1] + "/" + element.id)
}

setInterval(updateData, 500)