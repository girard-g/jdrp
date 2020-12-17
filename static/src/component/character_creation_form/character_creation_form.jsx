import React from 'react';
import Step1 from './step1';
import Step2 from './step2';
import Step3 from './step3';

class MasterForm extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            currentStep: 1,
            name: '',
            age: '',
            classs: '',
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
        }
    }

    handleChange = event => {
        const { name, value } = event.target
        this.setState({
            [name]: value
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
             `
        )
    }

    _next = () => {
        let currentStep = this.state.currentStep
        currentStep = currentStep >= 2 ? 3 : currentStep + 1
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

    /*
    * the functions for our button
    */
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
        if (currentStep < 3) {
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
                <p>Step {this.state.currentStep}/3 </p>
                <div class="container">
                    <div class="col-md-6 offset-md-3">
                        <form onSubmit={this.handleSubmit}>
                            <Step1
                                currentStep={this.state.currentStep}
                                handleChange={this.handleChange}
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
                                password={this.state.password}
                            />
                            {this.previousButton()}
                            {this.nextButton()}

                        </form>
                    </div>
                </div>


            </React.Fragment>
        );
    }
}

export default MasterForm;