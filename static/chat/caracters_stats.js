module.exports = function () {

    const stats = {
        "race": {
            "troll": {
                "strengh": 15,
                "education": -5,
                "charism": -10,
            },
            "elf": {
                "willpower": 10,
                "charism": 5,
                "force": -10,
            },
            "humain": {
                "dexterity": 10,
                "endurance": -10,
            },
            "demiElf": {
                "willpower": 5,
                "education": 5,
                "endurance": -5,
                "force": -5,
            },
            "nain": {
                "endurance": 5,
                "force": 5,
                "dexterity": -10,
                "charism": -5,
            },
            "saurial": {
                "dexterity": 5,
                "perception": 5,
                "charism": -5,
                "education": -5,
                "endurance": -5,
            },
        },
        "class": {
            "guerrier": {
                "force": 10,
                "endurance": 10,
                "willpower": -20,
            },
            "mage": {
                "willpower": 20,
                "force": -20,
                "education": 10,
            },
            "voleur": {
                "dexterity": 20,
                "force": -15,
                "endurance": -15,
            },
            "archer": {
                "dexterity": 10,
                "endurance": -10,
                "education": 5,
                "perception": 5,
            },
            "monk": {
                "force": 5,
                "dexterity": 5,
                "endurance": 5,
                "education": 5,
                "charism": -15,
            },
            "drood": {
                "force": 10,
                "willpower": 5,
                "charism": -5,
                "education": -5,
            },
            "paladin": {
                "endurance": 10,
                "willpower": 5,
                "dexterity": -10,
                "force": -5,
            },
            "alchimist": {
                "force": 15,
                "education": 15,
                "willpower": -10,
                "charism": 15,
                "endurance": -5,
            },
        }
    }

    return stats;
}