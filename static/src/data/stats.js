const stats = {
    description: "Est-ce qu'un personnage est musclé et perspicace ? Brillant et charmant ? Agile et robuste ? Les valeurs des caractéristiques définissent les qualités et les faiblesses d'une créature. Les trois principaux jets de dés du jeu (le jet de caractéristique, le jet de sauvegarde et le jet d'attaque) se basent sur les huits valeurs des caractéristiques.",
    strengh: {
        description: "La puissance physique, l'aptitude athlétique naturelle",
        mandatory: "guerrier, paladin, moine"
    },
    dexterity: {
        description: "L'agilité, les réflexes, l'équilibre",
        mandatory: "Moine, rôdeur, roublard"

    },
    endurance: {
        description: "La santé, l'endurance, la force vitale",
        mandatory: "Toutes les classes"

    },
    charism: {
        description: "La force de personnalité, l'éloquence, le leadership",
        mandatory: "Roublard"
    },
    perception: {
        description: "La perception, l'intuition, la perspicacité",
        mandatory: "Rôdeur, roublard"
    },
    luck: {
        description: "trait obscur laissé à la fantaisie du mj",
        mandatory: "Rôdeur, roublard"
    },
    willpower: {
        description: "L'acuité mentale, la force de caractère",
        mandatory: "Mage, clerc, paladin, druid"
    },
    education: {
        description: "Le raisonnement, la mémoire",
        mandatory: "Mage, clerc, druid, moine"

    },
}

export default stats