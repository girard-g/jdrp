import React, { useState, useEffect } from 'react';
import { useKeycloak } from '@react-keycloak/web';
import { Spinner } from 'react-bootstrap';
import MasterForm from '../../component/character_creation_form/character_creation_form.jsx';

const AppJdrp = () => {

  const keycloak = useKeycloak();

  const [error, setError] = useState(null);
  const [isLoaded, setisLoaded] = useState(false);
  const [rep, setrep] = useState(null);

  useEffect(() => {
    fetch("http://localhost:8000/api/getcharacter", {
      method: 'POST',
      body: keycloak.keycloak.token
    })
      .then(res => res.json())
      .then(
        (result) => {
          setisLoaded(true)
          setrep(result)
        },
        (error) => {
          setisLoaded(true)
          setError(error)
          console.log(error);
        }
      )
  });

  if (error) {
    return <div>Erreur : {error.message}</div>;
  } else if (!isLoaded) {
    return <Spinner animation="border" role="status" variant="primary"><span className="sr-only">Loading...</span></Spinner>;
  } else {
    if (rep === 'FALSE') {
      return <MasterForm />
    } else {
      return <span>toto</span>
    }
  }

};

export default AppJdrp;