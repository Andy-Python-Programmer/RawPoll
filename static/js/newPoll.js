const submitPoll = document.querySelector(".submit");

submitPoll.addEventListener("click", (event) => {
    event.preventDefault()

    const title = document.getElementById("title");
    const description = document.getElementById("description");

    const option_1 = document.getElementById("option-1");
    const option_2 = document.getElementById("option-2");
    
    fetch("/api/new?title=" + title.value + "&description=" + description.value + "&options=" + option_1.value + ": 1," + option_2.value + ": 1")
        .then(response => response.json())
        .then(data => {
            window.location.replace("/poll/" + data.id);
        });
});