"use strict";
var optionsDiv = document.querySelector(".options-all");
var count = 0;
for (var i = 0; i < 2; i++) {
    var form = document.createElement("div");
    count = i + 1;
    form.innerHTML = "\n\t<input class=\"form-control\" placeholder=\"Enter a option\" onfocus=\"addOption(this)\" onblur=\"removeOption(this)\">\n\t<br>\n\t";
    optionsDiv.appendChild(form);
}
function addOption(element) {
    var form = document.createElement("div");
    count = count + 1;
    form.innerHTML = "\n\t<input class=\"form-control\" placeholder=\"Enter a option\" onfocus=\"addOption(this)\" onblur=\"removeOption(this)\">\n\t<br>\n\t";
    if (element.parentNode.parentNode.lastElementChild === element.parentNode && element.parentNode.parentNode.firstChild.value !== "") {
        optionsDiv.appendChild(form);
    }
}
function removeOption(element) {
    if (element.value == "" && element.parentNode.parentNode.childNodes.length > 2) {
        element.parentNode.remove();
    }
}
var submitPoll = document.querySelector(".submit");
submitPoll.addEventListener("click", function (event) {
    event.preventDefault();
    var values = [];
    var ipCheck = document.querySelector("#ip-check").checked;
    var description = document.querySelector(".description").value;
    var question = document.querySelector(".question").value;
    optionsDiv.childNodes.forEach(function (element) {
        var val = element.children[0].value.trim();
        if (val !== "") {
            values.push(val);
        }
    });
    var data = {
        "question": question,
        "description": description,
        "options": values,
        "settings": {
            "ip-check": ipCheck
        }
    };
    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/api/poll/", true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.send(JSON.stringify(data));
    xhr.onload = function () {
        window.location.replace("/vote/" + JSON.parse(xhr.responseText).id);
    };
});
