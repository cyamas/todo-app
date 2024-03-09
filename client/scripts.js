function timeTask() {
    let activeTaskForm = document.getElementById("active-task-form");
    if (activeTaskForm.style.display != "none") {
        let durationInput = document.getElementById("active-task-duration");
        let currValue = parseInt(durationInput.value);
        durationInput.value = currValue + 1;
        console.log("test");
    }

}

function removeFromPending(id) {
    let todoId = "todo-" + id;
    console.log(todoId);
    let target = document.getElementById(todoId).remove();
    if (target) {
        target.parentNode.removeChild(target);
    }
}

function setPriorityColor(e) {
    let todo = e.target;
    todo.style.backgroundColor = todo.innerHTML;
    todo.innerHTML = ""
    console.log("this function works");
}

function toggleForm() {
    let form = document.getElementById("add-todo-form");
    if (form.style.display === "none") {
        form.style.display = "flex";
    } else {
        form.style.display = "none";
    }
}

function resetForm() {
    toggleForm();
    document.getElementById("add-todo-form").reset();
  }

function getTime() {
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]
    let currTime = new Date();
    let dayOfWeek = days[currTime.getDay()];
    let month = months[currTime.getMonth()];
    let dayOfMonth = currTime.getDate();
    let year = currTime.getFullYear();
    let hour = currTime.getHours();
    let amPm = "AM";
    if (hour > 12) {
        hour -= 12;
        amPm = "PM";
    } else if (hour == 0) {
        hour = 12;
    }
    let minute = currTime.getMinutes();
    if (minute < 10) {
        minute = "0" + minute;
    }
    let second = currTime.getSeconds();
    if (second < 10) {
        second = "0" + second;
    }

    document.getElementById("day-of-week").innerHTML = dayOfWeek;
    document.getElementById("date").innerHTML = month + " " + dayOfMonth + ", " + year;
    document.getElementById("time").innerHTML = hour + ":" + minute + " " + second + " " + amPm;
}

getTime();
setInterval(getTime, 1000);

setInterval(timeTask, 60000);