import React, { useEffect, useState } from 'react';
import Spinner from 'react-bootstrap/Spinner';
import Armor from './items/armor';
import Weapon from './items/weapon';
import Jewel from './items/jewel';
import Consumable from './items/consumable';

const Item = (props) => {

  const [error, setError] = useState(null);
  const [isLoaded, setIsLoaded] = useState(false);
  const [item, setItem] = useState(null);

  useEffect(() => {
    fetch("http://localhost:8000/api/testobjectgenerationlol")
      .then(res => res.json())
      .then(
        (result) => {
          setItem(result);
          setIsLoaded(true);
        },
        (error) => {
          setError(error);
          setIsLoaded(true);
        }
      )
  }, [])

  if (error) return <div>Erreur : {error.message}</div>
  if (!isLoaded) return <Spinner animation="border" role="status" variant="primary"><span className="sr-only">Loading...</span></Spinner>

  if (item.object !== null) {
    if (item.object.equipement.armor !== null) {
      return <Armor item={item} />
    } else if (item.object.equipement.weapon !== null) {
      return <Weapon item={item} />
    } else {
      return <Jewel item={item} />
    }
  } else {
    return <Consumable item={item} />
  }

}

export default Item