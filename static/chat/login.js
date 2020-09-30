// let submit = document.getElementById("submit");
// submit.addEventListener('click', function () {

//     let client = new HttpClient();
//     client.get('http://localhost:8000/login', function(response) {

//         if (response !== "") {
//             createAdminMessage("Last 5 messages:");
//             JSON.parse(response).forEach(function (item) {
//                 createHistoryMessage(item.body, item.author, item.published_at);
//             });
//             createAdminMessage("New Messages:")
//         }

//     });

// });