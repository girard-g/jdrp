import React from 'react';
import axios from 'axios';

class Board extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            // map: '',
            trad: {
                "warrior": "guerrier",
                "mage": "mage",
                "roublard": "roublard",
                "rodeur": "rodeur",
                "monk": "moine",
                "drood": "druid",
                "paladin": "paladin",
                "clerc": "clerc",
                "half_orc": "demi-orc",
                "elf": "elf",
                "human": "humain",
                "half_elf": "demi-elf",
                "dwarf": "nain",
                "saurial": "saurial",
                "cambion": "cambion",
            },
        }
    }

    // componentDidMount() {
    //     fetch("http://localhost:8000/static/images/map/Tds4.jpg")
    //         .then(
    //             (result) => {
    //                 this.setState({
    //                     map: result.url,

    //                 });
    //             },
    //             (error) => {
    //                 console.log(error)
    //             }
    //         )
    // }



    render() {
        return (
            <React.Fragment>
                <div className="container-fluid font-cursive">
                        {/* <div className="col-md-8" >
                        {/* <div className="col-md-8" style={{backgroungImage:`url(${this.state.map})`}}> */}
                        {/* <img className="img-fluid" src={this.state.map} alt="map" /> */}

                        <div class="justify-content-md-center">
                            <h1 className="text-center">Fiche Personnage</h1>
                            <p className="text-center">{this.props.c.name}, {this.state.trad[this.props.c.race]} {this.state.trad[this.props.c.class]}</p>

                            <div className="row">
                                <div className="card-body col-md-6">
                                    <p className="card-text text-center"><b>Alignement:</b> {this.props.c.alignment}</p>
                                    <p className="card-text text-center"><b>Particularité:</b> {this.props.c.particularity}</p>
                                </div>

                                <div className="card-body col-md-6">
                                    <p className="card-text text-center"><b>Age:</b> {this.props.c.age}</p>
                                    <p className="card-text text-center"><b>Réputation:</b> {this.props.c.reputation}</p>
                                </div>
                            </div>

                            <div className="row">
                                <div className="card-body col-md-6">
                                    <p className="card-text text-center"><b>Force:</b> {this.props.c.strengh}</p>
                                    <p className="card-text text-center"><b>Dextérité:</b> {this.props.c.dexterity}</p>
                                    <p className="card-text text-center"><b>Chance:</b> {this.props.c.luck}</p>
                                    <p className="card-text text-center"><b>Volonté:</b> {this.props.c.willpower}</p>
                                </div>

                                <div className="card-body col-md-6">
                                    <p className="card-text text-center"><b>Constitution:</b> {this.props.c.endurance}</p>
                                    <p className="card-text text-center"><b>Charisme:</b> {this.props.c.charism}</p>
                                    <p className="card-text text-center"><b>Perception:</b> {this.props.c.perception}</p>
                                    <p className="card-text text-center"><b>Education:</b> {this.props.c.education}</p>
                                </div>
                            </div>


                        </div>

                </div>

            </React.Fragment>
        );
    }
}

export default Board;