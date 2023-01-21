const data = {
    home: {
        hello: 'Bonjour !',
        intro: `Je suis développeur logiciel, voici mon cv.`,
        titleDev: 'Développement logiciel',
        software: {
            title: 'Développement logiciel',
            introduction: `Je suis développeur depuis 2016, actuellement chez [Ciné Digital](https://www.cinedigitalservice.com/cds/),
 je réalise actuellement l'application de vente internet multisite [Ciné Boutique](https://leplessisrobinsonmaisondesarts.cine.boutique/) depuis juillet 2018.
Je travaille donc sur des webapps, dans les technologies React/Redux, langages TypeScript / JavaScript / NodeJs occasionnellement Java, ExtJs et SQL. Je m'intéresse aussi au backend et à la programmation fonctionnelle (Elm, Rust). Mais pas que !`
        },
        wood: {
            title: 'Transformation du bois',
            introduction: `Dans une autre vie...
Mon parcours d'études et professionnel dans les métiers du bois m'a amené à éxercer le travail manuel du bois, l'apprentissage de l'utilisation des machines à bois, la théorie de la gestion de production et la pratique du dessin et de la conception assistée par ordinateur.
J'ai éxercé le métier de technicien (dessinateur, chef de projets en recherche et développement) dans les domaines de la menuiserie industrielle,
l'agencement mobilier et la conception de salles de bains en matériaux composites et tôle pliée. Mais ça, c'est du passé.`
        }
    },
    studies: {
        wood: [{
            beginning: '10/2011',
            duration: 1,
            end: '10/2012',
            icon: 'https://s3-eu-west-1.amazonaws.com/ctaque.logos/cci.jpg',
            id: 1,
            institution: 'Chambre de commerce et de l\'industrie',
            location: 'Lorient',
            text: 'Certificat de qualification RNCP niveau 3 technicien bureau d’études spécialisé CAO-DAO',
            courses: 'Formation aux logiciels : Autocad, Solidworks, CatiaV5, Inventor, Mécanique'
        }, {
            beginning: '09/2009',
            duration: 2,
            end: '06/2011',
            icon: 'https://s3-eu-west-1.amazonaws.com/ctaque.logos/esb.png',
            id: 2,
            institution: 'Ecole Supérieure du Bois',
            location: 'Nantes',
            text: 'BTS Productique bois option gestion de production industrielle',
            courses: 'Mécanique, qualité industrielle, gestion de production, dessin assisté par ordinateur, physique, mathématiques, Français - culture générale, automatisme, atelier de réalisation'
        }],
        software: [{
            beginning: '11/2016',
            duration: 0.8,
            end: '09/2017',
            icon: 'https://s3-eu-west-1.amazonaws.com/ctaque.logos/imie.png',
            id: 1,
            institution: 'IMIE',
            location: 'Nantes',
            text: 'Titre RNCP niveau 3 développeur logiciel',
            courses: 'Conception de bases de données (Merise, UML), Pattern MVC, Pattern event-emitter, POO, Sql, Php, Javascript, algorithmie'
        }]
    },
    experiences: {
        wood: [{
            beginning: '02/2015',
            contract: 'intérim',
            contract_type: 'travail',
            duration: 1,
            end: '02/2016',
            environment: 'industriel (CA 36M€ - fournisseur STX)',
            headline: 'Technicien bureau d’études',
            id: 1,
            institution: 'Altor Industrie',
            location: 'Clisson (44)',
            tasks: 'Mise à jour annuelle d\'un dossier de plan, amélioration et création de nouveaux modèles de cabines de bain en polyester et tôle d\'acier.',
            technologies: 'Solidworks 2014, EPDM, Cegid PMI',
        }, {
            beginning: '10/2013',
            contract: 'cdi',
            contract_type: 'travail',
            duration: 1,
            end: '10/2014',
            environment: 'industriel (CA 25M€)',
            headline: 'Chef de projets recherche et développement',
            id: 2,
            institution: 'Comec',
            location: 'La Tessoualle (49)',
            tasks: 'Création de plans pour l\'obtention d\'un procès verbal d\'autorisation de mise sur le marché, conception et dessin d\'un prototype de porte palière coupe feu et anti effraction, suivi de réalisation du prototype, contact fournisseurs, relevé de temps de fabrication pour l\'amélioration de l\'ERP interne, Logiciel: Topsolid V6',
            technologies: 'Topsolid V6',
        }, {
            beginning: '10/2011',
            contract: 'professionalisation',
            contract_type: 'alternance',
            duration: 1,
            end: '10/2012',
            environment: 'PME (CA 1M€)',
            headline: 'Technicien bureau d’études par alternance',
            id: 3,
            institution: 'Euroformes',
            location: 'Guichen (35)',
            tasks: 'Conception de mobilier bois, métal, Corian, verre, préparation des plans et documents de fabrication, appels fournisseurs. Logiciels Autocad, Solidworks, Rhinoceros',
            technologies: 'Rhinoceros, Solidworks, autocad',
        }, {
            beginning: '01/2011',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.4,
            end: '05/2011',
            environment: 'industriel',
            headline: 'Stage BTS',
            id: 4,
            institution: 'Pasquet Menuiseries',
            location: 'Ile et vilaine',
            tasks: 'Solutionnement d\'un aléa de production en menuiserie industrielle',
            technologies: 'Excel, Powerpoint',
        }, {
            beginning: '06/2010',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.3,
            end: '08/2010',
            environment: 'PME, petite industrie',
            headline: 'Stage de BTS en industrie de maisons modulaires.',
            id: 5,
            institution: 'Suprême Homes',
            location: 'Canada',
            tasks: 'Opérateur sur ligne de montage, solutionnement d\'un aléa de production',
            technologies: 'Excel',
        }, {
            beginning: '05/2009',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.2,
            end: '06/2009',
            environment: 'industriel',
            headline: 'Stage de terminale pro dans l\'agencement de luxe',
            id: 6,
            institution: 'ST Bois Concept',
            location: 'La Chevrolière (44)',
            tasks: 'Fabrication de prototypes de mobilier pour bateaux et camping-cars',
            technologies: 'Machines à bois d\'atelier, outillage électroportatif',
        }, {
            beginning: '12/2008',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.2,
            end: '01/2009',
            environment: 'industriel',
            headline: 'Stage de première année de Bac',
            id: 7,
            institution: 'Chantiers Bénéteau',
            location: 'Le Poiré sur Vie (85)',
            tasks: 'Opérateur sur ligne de montage des bateaux de la gamme Océanis',
            technologies: 'Outillage électroportatif',
        }],
        software: [{
            beginning: '01/2018',
            contract: 'cdi',
            contract_type: 'travail',
            duration: 0,
            end: null,
            environment: 'PME, Editeur de logiciels',
            headline: 'Développeur Webapps',
            id: 1,
            institution: 'Ciné Digital',
            location: 'La Chapelle sur Erdre (44)',
            tasks: `J\'interviens en tant que développeur FullStack Typescript/Node sur des thématiques de vente, dans une équipe de 6 développeurs : 

De la suite logicielle Ciné Office (point de vente caisse, borne, internet). 22 millions de tickets délivrés. Applications en monolithique et microservices.

De la plateforme de vente internet Ciné Boutique : Démarrage, développement, test et maintenance, évolution de la plateforme; un million de tickets délivrés en 3 ans; intégration du parcours d’achat dans des CMS; Paiement avec TPE Hipay, Paypal et Paybox. Gestion partielle de la montée en charge. (Front end Typescript/React, Backend NodeJs/MongoDB)

Réalisation de deux sites vitrine avec un CMS headless et un parcours d\'achat internet en Typescript, React et NodeJs.

Automatisation du déploiement continu des services en développement avec le CI/CD de Gitlab. 

Maintient à jour de la documentation développeur.`,
            technologies: 'Typescript, React, Jest, Webpack, Redux, Sass/Less, NodeJs, MongoDB, Postgresql, ExtJS, ERP Dolibarr, scheduler Rundeck; TPE Hipay, Paypal, Paybox; Outils : Ubuntu, Emacs, Docker compose, Gitlab, Gitlab CI/CD',
        }, {
            beginning: '06/2017',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.35,
            end: '09/2017',
            environment: 'startup',
            headline: 'Développeur full stack',
            id: 2,
            institution: 'Asmoza',
            location: 'Nantes',
            tasks: 'Mise en place d\'une architecture MVC et d\'un ORM dans une application',
            technologies: 'Backend php, SQL',
        }]
    },
    skills: {
        software: [
            {
                icon: 'https://cdn.svgporn.com/logos/elm.svg',
                id: 1,
                level: 'Quelques jours de pratique',
                name: 'Elm',
                technologies: [{
                    content: 'Développement de ce site et d\'un [jeu Pong](https://el-pong.netlify.app/) ',
                    id: 1,
                    name: 'Jeu/Front-end web'
                }]
            },
            {
                icon: 'https://upload.wikimedia.org/wikipedia/commons/a/ab/Cuddlyferris.svg',
                id: 2,
                level: 'Quelques semaines de pratique',
                name: 'Rust',
                technologies: [{
                    content: 'Développement du backend de cyprientaque.com',
                    id: 1,
                    name: 'Web Backend'
                }, {
                    content: 'Développement d\'une [librairie](https://ctprods.cyprientaque.com/blog/a-rust-api-pattern-actix) pour le framework Actix',
                    id: 2,
                    name: 'Lib'
                }]
            },
            {
                icon: 'https://cdn.svgporn.com/logos/typescript-icon.svg',
                id: 3,
                level: 'Quelques années de pratique',
                name: 'Typescript/React',
                technologies: [{
                    content: 'Démarrage, développement, maintenance, refonte graphique de la plateforme de réservation de places de cinéma Ciné Boutique. (Exemple de [site](https://guerandecinepresquile.cine.boutique/)). Un million de tickets délivrés.',
                    id: 1,
                    name: 'Ciné Boutique',
                }, {
                    content: 'Démarrage, livraison de deux sites vitrine et d\'un parcours d\'achat de billets de cinéma [www.cineum.fr](https://www.cineum.fr) (Cannes), [www.cineplanet.fr](https://www.cineplanet.fr)',
                    id: 2,
                    name: 'Cineum.fr - Cineplanet.fr'
                }, {
                    content: 'Démarrage d\'un projet d\'un tableau de bord d\'un système de gestion de cinéma (TMS, Theater Management System). Interfaçage avec un backend en Java.',
                    id: 3,
                    name: 'Hyperviseur de TMS'
                }, {
                    content: 'Développement web et maintenance de la suite logicielle ciné office. (22 millions de tickets délivrés).',
                    id: 4,
                    name: 'Ciné office',
                }]
            }
        ],
        wood: [{
            id: 2,
            name: 'CAO-DAO',
            level: 'Expérience de quelques années',
            technologies: [{
                id: 1,
                level: '',
                name: 'Logiciels: Solidworks, Topsolid, Epdm, Rhinoceros, Inventor',
            }]
        }]
    },
    hobbies: [
        {
            id: 1,
            name: 'Sport',
            content: [{
                content: 'Voile en [compétition](http://www.ffvoile.fr/ffv/sportif/ClmtCoureurFiche.asp?clid=1057876E) de 2002 à 2008 membre équipe régionale (dériveur).',
                id: 1,
            }, {
                content: 'Natation, marche à pieds..',
                id: 2,
            }]
        },
        {
            id: 2,
            name: 'Bricolage',
            content: [{
                content: 'Création de meubles et objets en bois.',
                id: 1,
            }, {
                content: 'Rénovation d’un fourgon aménagé : electricité, menuiserie.',
                id: 2,
            }]
        },
        {
            id: 3,
            name: 'Tourisme, voyages',
            content: [{
                content: 'Mer, montagne, Canada (stage de 5 mois en 2010), Etats unis, Europe, Afrique du nord... J\'aime la nature et la [photo](https://www.flickr.com/people/114643587@N06/).',
                id: 1,
            }]
        }, {
            id: 4,
            name: 'Culture',
            content: [{
                content: 'Livres sur les langages, Reddit, Romans (John Irving, Jules Verne), autobiographies (Lisa Jobs) ...',
                id: 1
            }, {
                content: 'Cinéma: cinéma d\'art et essai, (j\'adore !)',
                id: 2,
            }, {
                content: 'Musique: Electronique minimaliste, classique, folk, et un peu de TECHNO',
                id: 3,
            }]
        }
    ]

};

export default data;
