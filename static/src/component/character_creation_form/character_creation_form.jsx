import React from 'react';
import Step1 from './step1';
import Step2 from './step2';
import Step3 from './step3';
import Step4 from './step4';
import classes from '../../data/classes';
import races from '../../data/races';
import stats from '../../data/stats';
import axios from 'axios';
import ProgressBar from 'react-bootstrap/ProgressBar';

class MasterForm extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            currentStep: 1,
            name: '',
            age: '',
            classs: '',
            class_explanation: '-',
            race_explanation: '-',
            help: { name: '', description: '', mandatory: '' },
            race: '',
            alignment: '',
            particularity: '',
            reputation: '',
            strengh: 0,
            dexterity: 0,
            endurance: 0,
            charism: 0,
            perception: 0,
            luck: 0,
            willpower: 0,
            education: 0,
            class_strengh: 0,
            class_dexterity: 0,
            class_endurance: 0,
            class_charism: 0,
            class_perception: 0,
            class_luck: 0,
            class_willpower: 0,
            class_education: 0,
            race_strengh: 0,
            race_dexterity: 0,
            race_endurance: 0,
            race_charism: 0,
            race_perception: 0,
            race_luck: 0,
            race_willpower: 0,
            race_education: 0,
            portrait: '',
            config: '',
            total_point: 0,
            max_point: 0,
            min_point: 0,
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

    componentDidMount() {
        fetch("http://localhost:8000/api/config")
            .then(res => res.json())
            .then(
                (result) => {
                    this.setState({
                        config: result,
                        total_point: result.game_stats.max_stat_wcl,
                        max_point: result.game_stats.max_per_cat,
                        min_point: result.game_stats.min_per_cat,
                    });
                },
                (error) => {
                    console.log(error)
                }
            )
    }

    handleChange = event => {
        const { name, value } = event.target

        this.setState({
            [name]: value
        })
    }

    handleStatsChange = event => {
        const { name, value } = event.target

        console.log(name, value);

        let point = this.state.total_point - value;
        if (point < 0) {
            point = 0;
        }

        this.setState({
            [name]: value,
            total_point: point
        })
    }


    handleClassChange = event => {
        const { name, value } = event.target
        let val = ''
        let stats = ''

        if (value === "warrior") {
            val = classes.warrior.description;
            stats = this.state.config.class_stats.warrior
        } else if (value === "mage") {
            val = classes.mage.description;
            stats = this.state.config.class_stats.mage
        } else if (value === "roublard") {
            val = classes.roublards.description;
            stats = this.state.config.class_stats.roublard
        } else if (value === "rodeur") {
            val = classes.rodeur.description;
            stats = this.state.config.class_stats.rodeur
        } else if (value === "monk") {
            val = classes.monk.description;
            stats = this.state.config.class_stats.monk
        } else if (value === "drood") {
            val = classes.drood.description;
            stats = this.state.config.class_stats.drood
        } else if (value === "paladin") {
            val = classes.paladin.description;
            stats = this.state.config.class_stats.paladin
        } else if (value === "clerc") {
            val = classes.clerc.description;
            stats = this.state.config.class_stats.clerc
        } else {
            val = '-'
        }
        this.setState({
            [name]: value,
            class_explanation: val,
            class_strengh: stats.strengh,
            class_dexterity: stats.dexterity,
            class_luck: stats.luck,
            class_willpower: stats.willpower,
            class_endurance: stats.endurance,
            class_charism: stats.charism,
            class_perception: stats.perception,
            class_education: stats.education,
        })
    }

    handleRaceChange = event => {
        const { name, value } = event.target
        let val = ''
        let stats = ''

        if (value === "human") {
            val = races.human.description;
            stats = this.state.config.race_stats.human
        } else if (value === "half_orc") {
            val = races.half_orc.description;
            stats = this.state.config.race_stats.half_orc
        } else if (value === "elf") {
            val = races.elf.description;
            stats = this.state.config.race_stats.elf
        } else if (value === "half_elf") {
            val = races.half_elf.description;
            stats = this.state.config.race_stats.half_elf
        } else if (value === "dwarf") {
            val = races.dwarf.description;
            stats = this.state.config.race_stats.dwarf
        } else if (value === "saurial") {
            val = races.saurial.description;
            stats = this.state.config.race_stats.saurial
        } else if (value === "cambion") {
            val = races.cambion.description;
            stats = this.state.config.race_stats.cambion
        } else {
            val = '-'
        }
        this.setState({
            [name]: value,
            race_explanation: val,
            race_strengh: stats.strengh,
            race_dexterity: stats.dexterity,
            race_luck: stats.luck,
            race_willpower: stats.willpower,
            race_endurance: stats.endurance,
            race_charism: stats.charism,
            race_perception: stats.perception,
            race_education: stats.education,
        })
    }

    handleFocus = event => {
        const { name } = event.target
        let n = ''
        let val = ''
        let m = ''

        if (name === "strengh") {
            n = 'Force'
            m = 'Important pour ' + stats.strengh.mandatory
            val = stats.strengh.description;
        } else if (name === "dexterity") {
            n = 'Dextérité'
            m = 'Important pour ' + stats.dexterity.mandatory
            val = stats.dexterity.description;
        } else if (name === "endurance") {
            n = 'Constitution'
            m = 'Important pour ' + stats.endurance.mandatory
            val = stats.endurance.description;
        } else if (name === "charism") {
            n = 'Charisme'
            m = 'Important pour ' + stats.charism.mandatory
            val = stats.charism.description;
        } else if (name === "perception") {
            n = 'Perception'
            m = 'Important pour ' + stats.perception.mandatory
            val = stats.perception.description;
        } else if (name === "luck") {
            n = 'Chance'
            m = 'Important pour ' + stats.luck.mandatory
            val = stats.luck.description;
        } else if (name === "willpower") {
            n = 'Volonté'
            m = 'Important pour ' + stats.willpower.mandatory
            val = stats.willpower.description;
        } else if (name === "education") {
            n = 'Education'
            m = 'Important pour ' + stats.education.mandatory
            val = stats.education.description;
        } else {
            n = '-'
            m = '-'
            val = '-'
        }
        this.setState({
            help: {
                name: n,
                description: val,
                mandatory: m
            }
        })
    }

    handleOnClick = event => {
        const { name, value } = event.target

        let t = this.state.total_point - parseInt(value)

        if (t < 0) {
            t = 0;
            return false
        } else if (t > this.state.config.game_stats.max_stat_wcl) {
            t = this.state.config.game_stats.max_stat_wcl;
            return false
        }


        if (name === "strengh") {
            this.setState({
                strengh: this.state.strengh + parseInt(value),
                total_point: t
            })
        } else if (name === "dexterity") {
            this.setState({
                dexterity: this.state.dexterity + parseInt(value),
                total_point: t
            })
        } else if (name === "endurance") {
            this.setState({
                endurance: this.state.endurance + parseInt(value),
                total_point: t
            })
        } else if (name === "charism") {
            this.setState({
                charism: this.state.charism + parseInt(value),
                total_point: t
            })
        } else if (name === "perception") {
            this.setState({
                perception: this.state.perception + parseInt(value),
                total_point: t
            })
        } else if (name === "luck") {
            this.setState({
                luck: this.state.luck + parseInt(value),
                total_point: t
            })
        } else if (name === "willpower") {
            this.setState({
                willpower: this.state.willpower + parseInt(value),
                total_point: t
            })
        } else if (name === "education") {
            this.setState({
                education: this.state.education + parseInt(value),
                total_point: t
            })
        } else {

        }
    }

    handleSubmit = event => {
        event.preventDefault()
        const {
            name,
            age,
            classs,
            race,
            alignment,
            particularity,
            reputation,
            strengh,
            dexterity,
            endurance,
            charism,
            perception,
            luck,
            willpower,
            education,
            portrait,
        } = this.state

        let user_input = {
            name: name,
            age: parseInt(age),
            class: classs,
            race: race,
            alignment: alignment,
            particularity: particularity,
            reputation: reputation,
            strengh: parseInt(strengh),
            dexterity: parseInt(dexterity),
            endurance: parseInt(endurance),
            charism: parseInt(charism),
            perception: parseInt(perception),
            luck: parseInt(luck),
            willpower: parseInt(willpower),
            education: parseInt(education),
            portrait: portrait,
        }

        axios.post("http://localhost:8000/api/check-caracter-creation", user_input, {
            headers: {
                'Authorization': this.props.k.keycloak.token,
                // 'content-type': 'application/json'
            },
        })
            .then(
                response => (
                    // setisLoaded(true),
                    console.log(response.data)
                ),
                (error) => {
                    console.log(error)
                }
            )
    }

    _next = () => {
        let currentStep = this.state.currentStep
        currentStep = currentStep >= 3 ? 4 : currentStep + 1
        this.setState({
            currentStep: currentStep
        })

        if (currentStep === 2) {
            this.setState({
                strengh: this.state.class_strengh + this.state.race_strengh,
                dexterity: this.state.class_dexterity + this.state.race_dexterity,
                luck: this.state.class_luck + this.state.race_luck,
                willpower: this.state.class_willpower + this.state.race_willpower,
                endurance: this.state.class_endurance + this.state.race_endurance,
                charism: this.state.class_charism + this.state.race_charism,
                perception: this.state.class_perception + this.state.race_perception,
                education: this.state.class_education + this.state.race_education,
            })
        }
    }

    _prev = () => {
        let currentStep = this.state.currentStep
        currentStep = currentStep <= 1 ? 1 : currentStep - 1
        this.setState({
            currentStep: currentStep
        })

        if (currentStep === 1) {
            this.setState({
                strengh: 0,
                dexterity: 0,
                luck: 0,
                willpower: 0,
                endurance: 0,
                charism: 0,
                perception: 0,
                education: 0,
                total_point: this.state.config.game_stats.max_stat_wcl
            })
        }
    }

    previousButton() {
        let currentStep = this.state.currentStep;
        if (currentStep !== 1) {
            return (
                <button
                    className="btn btn-secondary"
                    type="button" onClick={this._prev}>
                    Previous
                </button>
            )
        }
        return null;
    }

    nextButton() {
        let currentStep = this.state.currentStep;
        if (currentStep < 4) {
            return (
                <button
                    className="btn btn-primary float-right"
                    type="button" onClick={this._next}>
                    Next
                </button>
            )
        }
        return null;
    }

    render() {
        return (
            <React.Fragment>
                <ProgressBar now={this.state.currentStep * 25} label={`${this.state.currentStep}/4`} />
                <h1>Create your character 🧙‍♂️</h1>
                <div className="container">
                    <div className="row">
                        <div className="col-md-4">
                            <form onSubmit={this.handleSubmit} className="needs-validation" novalidate>
                                <Step1
                                    currentStep={this.state.currentStep}
                                    handleChange={this.handleChange}
                                    handleClassChange={this.handleClassChange}
                                    handleRaceChange={this.handleRaceChange}
                                    email={this.state.email}
                                    age={this.state.age}
                                    classs={this.state.classs}
                                    race={this.state.race}
                                    alignment={this.state.alignment}
                                    particularity={this.state.particularity}
                                    reputation={this.state.reputation}
                                />
                                <Step2
                                    currentStep={this.state.currentStep}
                                    handleChange={this.handleStatsChange}
                                    handleFocus={this.handleFocus}
                                    onclickHandler={this.handleOnClick}
                                    strengh={this.state.strengh}
                                    strenghHelper={stats.strengh.description}
                                    strenghMandatory={stats.strengh.mandatory}
                                    dexterity={this.state.dexterity}
                                    dexterityHelper={stats.dexterity.description}
                                    dexterityMandatory={stats.dexterity.mandatory}
                                    endurance={this.state.endurance}
                                    enduranceHelper={stats.endurance.description}
                                    enduranceMandatory={stats.endurance.mandatory}
                                    charism={this.state.charism}
                                    charismHelper={stats.charism.description}
                                    charismMandatory={stats.charism.mandatory}
                                    perception={this.state.perception}
                                    perceptionHelper={stats.perception.description}
                                    perceptionMandatory={stats.perception.mandatory}
                                    luck={this.state.luck}
                                    luckHelper={stats.luck.description}
                                    luckMandatory={stats.luck.mandatory}
                                    willpower={this.state.willpower}
                                    willpowerHelper={stats.willpower.description}
                                    willpowerMandatory={stats.willpower.mandatory}
                                    education={this.state.education}
                                    educationHelper={stats.education.description}
                                    educationMandatory={stats.education.mandatory}
                                    min_point={this.state.min_point}
                                    max_point={this.state.max_point}
                                />
                                <Step3
                                    currentStep={this.state.currentStep}
                                    handleChange={this.handleChange}
                                    portrait={this.state.portrait}
                                />
                                <Step4
                                    currentStep={this.state.currentStep}
                                    handleChange={this.handleChange}
                                />
                                {this.previousButton()}
                                {this.nextButton()}

                            </form>
                        </div>

                        {this.state.currentStep !== 4 &&
                            <div className="card border border-info border-5 col-md-8 overflow-auto" style={{ minHeight: 600, color: 'black' }}>
                                <img src="https://via.placeholder.com/538x107/" className="card-img-top" alt="Class Banner"></img>
                                <div className="card-body">
                                    {this.state.currentStep === 1 &&
                                        <>
                                            {this.state.race !== "" &&
                                                <>
                                                    <h2 className="card-title text-center">Race</h2>
                                                    <p className="card-text">{this.state.race_explanation}</p>
                                                    <table class="table">
                                                        <thead>
                                                            <tr>
                                                                <th scope="col">Strengh</th>
                                                                <th scope="col">Dexterity</th>
                                                                <th scope="col">Luck</th>
                                                                <th scope="col">Willpower</th>
                                                                <th scope="col">Endurance</th>
                                                                <th scope="col">Charism</th>
                                                                <th scope="col">Perception</th>
                                                                <th scope="col">Education</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>

                                                            <tr>
                                                                <th scope="row">{this.state.race_strengh}</th>
                                                                <th scope="row">{this.state.race_dexterity}</th>
                                                                <th scope="row">{this.state.race_luck}</th>
                                                                <th scope="row">{this.state.race_willpower}</th>
                                                                <th scope="row">{this.state.race_endurance}</th>
                                                                <th scope="row">{this.state.race_charism}</th>
                                                                <th scope="row">{this.state.race_perception}</th>
                                                                <th scope="row">{this.state.race_education}</th>
                                                            </tr>
                                                        </tbody>
                                                    </table>
                                                </>
                                            }
                                            {this.state.classs !== "" &&
                                                <>
                                                    <h2 className="card-title text-center">Classe</h2>
                                                    <p className="card-text">{this.state.class_explanation}</p>

                                                    <table class="table">
                                                        <thead>
                                                            <tr>
                                                                <th scope="col">Strengh</th>
                                                                <th scope="col">Dexterity</th>
                                                                <th scope="col">Luck</th>
                                                                <th scope="col">Willpower</th>
                                                                <th scope="col">Endurance</th>
                                                                <th scope="col">Charism</th>
                                                                <th scope="col">Perception</th>
                                                                <th scope="col">Education</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            <tr>
                                                                <th scope="row">{this.state.class_strengh}</th>
                                                                <th scope="row">{this.state.class_dexterity}</th>
                                                                <th scope="row">{this.state.class_luck}</th>
                                                                <th scope="row">{this.state.class_willpower}</th>
                                                                <th scope="row">{this.state.class_endurance}</th>
                                                                <th scope="row">{this.state.class_charism}</th>
                                                                <th scope="row">{this.state.class_perception}</th>
                                                                <th scope="row">{this.state.class_education}</th>
                                                            </tr>
                                                        </tbody>
                                                    </table>
                                                </>
                                            }
                                        </>
                                    }

                                    {this.state.currentStep === 2 &&
                                        <>
                                            <p className="card-text">{stats.description}</p>
                                            <p className="card-text"><b>{this.state.total_point}</b> point(s) restant à répartir</p>
                                            <p className="card-text small">Chaque statistique ne peut être inférieure à <b>{this.state.min_point}</b> ni dépasser <b>{this.state.max_point}</b></p>

                                        </>
                                    }
                                    <hr />
                                    <h4 className="card-title text-center">Fiche Personnage</h4>
                                    <p className="card-text text-center">{this.state.name}</p>

                                    <div className="row">
                                        <div className="card-body col-md-6">
                                            <p className="card-text text-center"><b>Race:</b> {this.state.trad[this.state.race]}</p>
                                            <p className="card-text text-center"><b>Alignement:</b> {this.state.alignment}</p>
                                            <p className="card-text text-center"><b>Particularité:</b> {this.state.particularity}</p>
                                        </div>

                                        <div className="card-body col-md-6">
                                            <p className="card-text text-center"><b>Classe:</b> {this.state.trad[this.state.classs]}</p>
                                            <p className="card-text text-center"><b>Age:</b> {this.state.age}</p>
                                            <p className="card-text text-center"><b>Réputation:</b> {this.state.reputation}</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }

                        {this.state.currentStep === 4 &&
                            <div className="card col-md-6 overflow-auto " style={{ height: 600, maxHeight: 600, color: 'black' }}>
                                <img src="https://via.placeholder.com/150x250/" className="card-img" alt="portrait"></img>
                                <div className="card-body">

                                    <div className="card-body">
                                        <h2 className="card-title text-center">Fiche Personnage</h2>
                                        <p className="card-text text-center">{this.state.name}</p>

                                        <div className="row">
                                            <div className="card-body col-md-6">
                                                <p className="card-text text-center"><b>Race:</b> {this.state.trad[this.state.race]}</p>
                                                <p className="card-text text-center"><b>Alignement:</b> {this.state.alignment}</p>
                                                <p className="card-text text-center"><b>Particularité:</b> {this.state.particularity}</p>
                                            </div>

                                            <div className="card-body col-md-6">
                                                <p className="card-text text-center"><b>Classe:</b> {this.state.trad[this.state.classs]}</p>
                                                <p className="card-text text-center"><b>Age:</b> {this.state.age}</p>
                                                <p className="card-text text-center"><b>Réputation:</b> {this.state.reputation}</p>
                                            </div>
                                        </div>
                                    </div>


                                </div>
                            </div>
                        }
                    </div>
                </div>
            </React.Fragment>
        );
    }
}

export default MasterForm;