const Home = (key) => {

    console.log(key);
    return <span>{key.keycloak.idToken}</span>;
}

export default Home;