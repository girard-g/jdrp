import React from 'react';
import Step1 from './step1';
import Step2 from './step2';
import Step3 from './step3';
import Step4 from './step4';
import classes from '../../data/classes';
import races from '../../data/races';
import stats from '../../data/stats';

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
            help: { name: '', desciprtion: '', mandatory: '' },
            race: '',
            alignment: '',
            particularity: '',
            reputation: '',
            strengh: '',
            dexterity: '',
            endurance: '',
            charism: '',
            perception: '',
            luck: '',
            willpower: '',
            education: '',
            portrait: '',
        }
    }

    handleChange = event => {
        const { name, value } = event.target

        console.log(name,value );
        this.setState({
            [name]: value
        })
    }

    handleClassChange = event => {
        const { name, value } = event.target
        let val = ''
        if (value === "warrior") {
            val = classes.warrior.description;
        } else if (value === "mage") {
            val = classes.mage.description;
        } else if (value === "roublard") {
            val = classes.roublards.description;
        } else if (value === "rodeur") {
            val = classes.rodeur.description;
        } else if (value === "monk") {
            val = classes.monk.description;
        } else if (value === "drood") {
            val = classes.drood.description;
        } else if (value === "paladin") {
            val = classes.paladin.description;
        } else if (value === "clerc") {
            val = classes.clerc.description;
        } else {
            val = '-'
        }
        this.setState({
            [name]: value,
            class_explanation: val
        })
    }

    handleRaceChange = event => {
        const { name, value } = event.target
        let val = ''
        if (value === "human") {
            val = races.human.description;
        } else if (value === "half_orc") {
            val = races.half_orc.description;
        } else if (value === "elf") {
            val = races.elf.description;
        } else if (value === "half_elf") {
            val = races.half_elf.description;
        } else if (value === "dwarf") {
            val = races.dwarf.description;
        } else if (value === "saurial") {
            val = races.saurial.description;
        } else if (value === "cambion") {
            val = races.cambion.description;
        } else {
            val = '-'
        }
        this.setState({
            [name]: value,
            race_explanation: val
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
            val = stats.dexterity.description;
        } else if (name === "endurance") {
            n = 'Constitution'
            val = stats.endurance.description;
        } else if (name === "charism") {
            n = 'Charisme'
            val = stats.charism.description;
        } else if (name === "perception") {
            n = 'Perception'
            val = stats.perception.description;
        } else if (name === "luck") {
            n = 'Chance'
            val = stats.luck.description;
        } else if (name === "willpower") {
            n = 'Volonté'
            val = stats.willpower.description;
        } else if (name === "education") {
            n = 'Education'
            val = stats.education.description;
        } else {
            n = '-'
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
        alert(`Your registration detail: \n 
             name: ${name} \n 
             age: ${age} \n 
             classs: ${classs} \n 
             race: ${race} \n 
             alignment: ${alignment} \n 
             particularity: ${particularity} \n 
             reputation: ${reputation} \n
             strengh: ${strengh} \n 
             dexterity: ${dexterity} \n 
             endurance: ${endurance} \n 
             charism: ${charism} \n 
             perception: ${perception} \n 
             luck: ${luck} \n 
             willpower: ${willpower} \n 
             education: ${education} \n 
             portrait: ${portrait} \n 
             `
        )
    }



    _next = () => {
        let currentStep = this.state.currentStep
        currentStep = currentStep >= 3 ? 4 : currentStep + 1
        this.setState({
            currentStep: currentStep
        })
    }

    _prev = () => {
        let currentStep = this.state.currentStep
        currentStep = currentStep <= 1 ? 1 : currentStep - 1
        this.setState({
            currentStep: currentStep
        })
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
                <h1>Create your character 🧙‍♂️</h1>
                <p>Step {this.state.currentStep}/4 </p>
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
                                    handleChange={this.handleChange}
                                    handleFocus={this.handleFocus}
                                    strengh={this.state.strengh}
                                    dexterity={this.state.dexterity}
                                    endurance={this.state.endurance}
                                    charism={this.state.charism}
                                    perception={this.state.perception}
                                    luck={this.state.luck}
                                    willpower={this.state.willpower}
                                    education={this.state.education}
                                />
                                <Step3
                                    currentStep={this.state.currentStep}
                                    handleChange={this.handleChange}
                                    portrait={this.state.portrait}
                                />
                                <Step4
                                    currentStep={this.state.currentStep}
                                    handleChange={this.handleChange}
                                    password={this.state.password}
                                />
                                {this.previousButton()}
                                {this.nextButton()}

                            </form>
                        </div>
                    
                        <div className="card col-md-4 overflow-auto " style={{ height: 600, maxHeight: 600, color: 'black' }}>
                            <img src="https://via.placeholder.com/538x107/" className="card-img-top" alt="Class Banner"></img>
                            {this.state.currentStep === 1 &&
                                <div className="card-body">
                                    <h2 className="card-title text-center">Race</h2>
                                    <p className="card-text">{this.state.race_explanation}</p>
                                    <h2 className="card-title text-center">Classe</h2>
                                    <p className="card-text">{this.state.class_explanation}</p>
                                </div>
                            }
                            {this.state.currentStep === 2 &&
                                <div className="card-body">
                                    <p className="card-text">{stats.description}</p>
                                    <h4 className="card-title text-center">{this.state.help.name}</h4>
                                    <p className="card-text">{this.state.help.description}</p>
                                    <p className="card-text"><i>{this.state.help.mandatory}</i></p>
                                </div>
                            }
                             {this.state.currentStep === 3 &&
                                <div className="card-body">
                                    <p className="card-text">{stats.description}</p>
                                    <h4 className="card-title text-center">{this.state.help.name}</h4>
                                    <p className="card-text">{this.state.help.description}</p>
                                    <p className="card-text"><i>{this.state.help.mandatory}</i></p>
                                </div>
                            }
                        </div>

                        <div className="card col-md-4 overflow-auto " style={{ height: 600, maxHeight: 600, color: 'black' }}>
                            <img src="https://via.placeholder.com/538x700/" className="card-img-top" alt="portrait"></img>
                            <div className="card-body">
                                <h2 className="card-title text-center">Lore</h2>
                                <p className="card-text">TODO</p>
                            </div>
                        </div>
                        
                    </div>

                </div>


            </React.Fragment>
        );
    }
}

export default MasterForm;