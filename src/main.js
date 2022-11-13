const { listen, emit } = window.__TAURI__.event;
const { invoke } = window.__TAURI__.tauri;

listen("auth_codes", (event) => {
  console.log(event.payload);
  codes = event.payload;
    //loop through codes and display them
  //clear the greet-mgs
  document.getElementById("codes-msg").innerHTML = "";
  codes.forEach(element => {
    //create a new div
    let newDiv = document.createElement("div");
    //create a new text node
    let newContent = document.createTextNode(element[0] + " : " + element[1]);
    //add the text node to the newly created div
    newDiv.appendChild(newContent);
    //add the newly created element and its content into the DOM
    let currentDiv = document.getElementById("codes-msg");

    currentDiv.appendChild(newDiv);

  });
});

function notify(message) {
  emit("auth_codes", { "hello": "world" });

}
async function heartbeat() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let codes = await invoke("heartbeat");

}
//when the page loads
window.onload = function () {

  //get the button
  let button = document.getElementById("confirmations-btn");
  //add an event listener to the button
  button.addEventListener("click", get_trade_confirmations);

}


async function get_trade_confirmations() {
  let confirmations = await invoke("get_trade_confirmations", { accountName: document.getElementById("confirm_user").value, password: document.getElementById("confirm_pass").value });
  console.log(confirmations);
  document.getElementById("confirmations-msg").innerHTML = "";
  confirmations.forEach(element => {
    //create a new div
    let newDiv = document.createElement("div");
    //create a new text node
    let newContent = document.createTextNode(element[0] + " : " + element[1]);
    //add the text node to the newly created div
    newDiv.appendChild(newContent);
    //add the newly created element and its content into the DOM
    let currentDiv = document.getElementById("confirmations-msg");

    currentDiv.appendChild(newDiv);

  });
}
// get_trade_confirmations()
//call greet every 5 seconds
setInterval(heartbeat, 1000);
heartbeat()