import Keycloak from 'keycloak-js';

const initOptions = {
    url: 'http://127.0.0.1:8080/auth', realm: 'JDRP', clientId: 'JDRPClient', onLoad: 'login-required'
  }

const keycloak = new Keycloak(initOptions);
export default keycloak

// keycloak.init({ onLoad: initOptions.onLoad }).then((auth) => {
// //   if (!auth) {
// //     window.location.reload();
// //   } else {
//     console.log("Authenticated");
//    alert(auth);


// //   }
// })