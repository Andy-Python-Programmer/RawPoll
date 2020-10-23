const body = document.querySelector("body");
const opts = document.querySelector(".opts");
const id = window.location.href.split("/poll/");

var prevData = {};
var curCharInstance;

Object.size = function(obj) {
    var size = 0, key;
    for (key in obj) {
        if (obj.hasOwnProperty(key)) size++;
    }
    return size;
};

function chartShow(data) {
    let pollChart = document.getElementById("poll").getContext('2d');
     
    if (curCharInstance != null) {
        curCharInstance.destroy();
    }

    pollChart.canvas.parentNode.style.height = window.innerHeight / 2 + "px";
    pollChart.canvas.parentNode.style.width = window.innerHeight / 2 + "px";

    curCharInstance = new Chart(pollChart, {
        type: 'doughnut',
        data: {
            labels: Object.keys(data.options),
            datasets: [ {
                data: Object.values(data.options),
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
        var refresh;

        if (JSON.stringify(data.options) === JSON.stringify(prevData)) {
            refresh = false;
        }

        else {
            refresh = true;
        }

        if (refresh) {
            prevData = data.options;

            chartShow(data);
        }
    })
}

function copy() {
    var copyText = document.getElementById("copy-input");
    copyText.select();
    copyText.setSelectionRange(0, 99999);
    document.execCommand("copy");
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
                data-toggle="tooltip" data-placement="button" onClick="copy()"
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

        for (var i = 0; i < Object.size(data.options); i++) {
            const value = Object.keys(data.options)[i];

            console.log(value);
    
            opts.innerHTML += `
            <button class="btn btn-primary" onClick="update(this)" id="${value}">${value}</button>
            `
        };
    });
}

main();

function update(element) {
    fetch(id[0] + "/api/poll/" + id[1] + "/" + element.id)
}

setInterval(updateData, 500)