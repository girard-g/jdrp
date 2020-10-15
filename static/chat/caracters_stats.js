module.exports = function () {

    const stats = {
        "race": {
            "demiOrc": {
                "force": 15,
                "endurance": 10,
                "charism": -10,
                "education": -5,
            },
            "elf": {
                "force": -10,
                "dexterity": 5,
                "willpower": 10,
                "charism": 5,
            },
            "humain": {
                "dexterity": 10,
                "luck": 5,
                "endurance": -10,
                "charism": 5,
            },
            "demiElf": {
                "force": -5,
                "dexterity": 5,
                "willpower": 5,
                "endurance": -5,
                "charism": 5,
                "education": 5,
            },
            "nain": {
                "force": 10,
                "dexterity": -10,
                "willpower": 5,
                "endurance": 5,
                "charism": -5,
                "perception": 5,
            },
            "saurial": {
                "dexterity": 5,
                "luck": 5,
                "endurance": -5,
                "charism": -5,
                "perception": 10,
            },
            "cambion": {
                "force": 5,
                "dexterity": 5,
                "luck": 5,
                "willpower": 10,
                "charism": -10,
                "education": -5,
            },
        },
        "class": {
            "guerrier": {
                "force": 10,
                "dexterity": 5,
                "willpower": -15,
                "endurance": 10,
            },
            "mage": {
                "force": -20,
                "willpower": 20,
                "education": 10,
            },
            "roublard": {
                "force": -15,
                "dexterity": 15,
                "luck": 5,
                "willpower": 5,
                "endurance": -10,
                "perception": 10,
            },
            "rodeur": {
                "dexterity": 10,
                "luck": 5,
                "endurance": -10,
                "perception": 5,
            },
            "monk": {
                "force": 5,
                "dexterity": 5,
                "endurance": 5,
                "charism": -10,
                "education": 5,
            },
            "drood": {
                "force": 5,
                "dexterity": -10,
                "willpower": 5,
                "endurance": 10,
            },
            "paladin": {
                "force": 10,
                "dexterity": -5,
                "willpower": 5,
                "endurance": 5,
                "perception": -5,
            },
            "clerc": {
                "force": 5,
                "dexterity": -5,
                "willpower": 10,
                "endurance": -10,
                "education": 10,
            },
        }
    }

    return stats;
}