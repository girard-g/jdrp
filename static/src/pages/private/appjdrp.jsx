import React, { useState, useEffect } from 'react';
import { useKeycloak } from '@react-keycloak/web';
import { Spinner } from 'react-bootstrap';
import axios from 'axios';
import CTabs from '../../component/board/CTabs';
import MasterForm from '../../component/character_creation_form/character_creation_form.jsx';

const AppJdrp = () => {

    const keycloak = useKeycloak();

    const [error, setError] = useState(null);
    const [isLoaded, setisLoaded] = useState(false);
    const [rep, setRep] = useState({});

    useEffect(() => {
        axios.post("http://localhost:8000/api/getcharacter", {
            withCredentials: true,
            credentials: 'include',
        }, {
            headers: { 'Authorization': keycloak.keycloak.token }
        })
            .then(
                response => (
                    setisLoaded(true),
                    setRep(JSON.parse(response.data)),
                    setTimeout(3000)

                ),
                (error) => {
                    setisLoaded(true)
                    setError(error)
                    console.log(error);
                }
            )
    }, []);

    if (error) {
        return <div>Erreur : {error.message}</div>;
    } else if (!isLoaded) {
        return <Spinner animation="border" role="status" variant="primary"><span className="sr-only">Loading...</span></Spinner>;
    } else {
        if (rep === 'FALSE') {
            return <MasterForm k={keycloak} />
        } else {
            return <CTabs c={rep} />
        }
    }

};

export default AppJdrp;