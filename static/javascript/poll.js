"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var id = window.location.pathname.split("/");
id = id[id.length - 1];
var title = document.querySelector(".title");
var description = document.querySelector(".description");
var options = document.querySelector(".options");
var prevPoll;
fetch("/api/poll/" + id)
    .then(response => {
    response.json().then(function (data) {
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
        });
    });
});
function isEquivalent(a, b) {
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
        if (a[propName] !== b[propName]) {
            return false;
        }
    }
    // If we made it this far, objects
    // are considered equivalent
    return true;
}
setInterval(() => __awaiter(void 0, void 0, void 0, function* () {
    const response = yield fetch("/api/poll/" + id);
    const poll = yield response.json();
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
        });
    }
    ;
}), 500);
