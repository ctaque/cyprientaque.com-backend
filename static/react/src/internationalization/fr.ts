const data = {
    home: {
        about: '',
        titleDev: 'Développement logiciel',
        software: {
            title: 'Un développeur passionné et polyvalent avec une expertise dans les logiciels et l\'environnement GNU/Linux',
            introduction: `Je suis un développeur passionné par les logiciels et l'environnement GNU/Linux, qui possède une vaste expérience dans différents domaines. J'ai travaillé sur des projets de développement web en utilisant React, Typescript et Sass, des applications mobiles en utilisant Flutter, Dart et Capacitor, et du développement backend en utilisant NodeJs et Rust. Je suis également compétent dans l'utilisation de bases de données telles que Postgresql, MongoDB et MySQL. En dehors de cela, j'ai également travaillé sur des projets variés tels que des web apps, des sites vitrines, des applications mobiles, ainsi que la personnalisation de mon environnement de développement Linux.`
        },
        wood: {
            title: 'Parcours académique et professionnel dans les métiers du bois',
            introduction: `Mon parcours académique et professionnel dans le domaine de la menuiserie m'a permis d'acquérir de solides compétences pratiques telles que le travail manuel du bois et l'utilisation de machines à bois. J'ai également acquis des connaissances théoriques en gestion de production ainsi que des compétences en dessin et en conception assistée par ordinateur. Au cours de ma carrière, j'ai occupé le poste de technicien, assumant les rôles de dessinateur et de chef de projets en recherche et développement. J'ai travaillé dans les secteurs de la menuiserie industrielle, de l'agencement mobilier et de la conception de salles de bains en matériaux composites et en tôle pliée.`
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
            tasks: 'Dans le cadre de mes fonctions, j\'ai été en charge de la mise à jour annuelle d\'un dossier de plan, ainsi que l\'amélioration et la création de nouveaux modèles de cabines de bain en polyester et tôle d\'acier. J\'ai travaillé à l\'aide des technologies Solidworks 2014, EPDM et Cegid PMI.',
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
            tasks: 'Ce projet a impliqué plusieurs tâches telles que l\'élaboration de plans pour l\'obtention d\'une autorisation de mise sur le marché, la conception et le dessin d\'un prototype de porte palière coupe-feu et anti-effraction, ainsi que le suivi de sa réalisation. En outre, il a inclut également la gestion des contacts fournisseurs, le suivi du temps de fabrication pour améliorer l\'ERP interne et l\'utilisation du logiciel Topsolid V6.',
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
            tasks: 'J\'étais chargé de la conception de mobilier en bois, en métal, en Corian et en verre, ainsi que de la préparation des plans et des documents nécessaires à la fabrication. Mon travail comprenait également la gestion des appels fournisseurs. Pour mener à bien ces tâches, j\'utilisais les logiciels Autocad, Solidworks, Rhinoceros et le logiciel Excel.',
            technologies: 'Rhinoceros, Solidworks, Autocad, Excel',
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
            tasks: 'Dans le cadre de mes fonctions en menuiserie industrielle, j\'ai été chargé de résoudre un aléa de production.',
            technologies: 'Logiciels Excel, Powerpoint',
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
            tasks: 'Mon travail consistait à participer à la construction de maisons en tant qu\'opérateur sur la ligne de montage. Pendant mon stage, j\'ai également été chargé de résoudre un aléa de production.',
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
            tasks: 'Pendant mon stage, j\'ai eu l\'opportunité de fabriquer des prototypes de mobilier destinés aux bateaux et aux camping-cars.',
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
            tasks: 'Pendant mon stage, j\'ai eu l\'opportunité d\'être opérateur sur la ligne de montage des bateaux de la gamme Océanis',
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
            tasks: `Je travaille en tant que développeur FullStack Typescript/Node au sein d'une équipe de 6 développeurs. Mes interventions portent sur des thématiques de vente, notamment dans la suite logicielle Ciné Office qui comprend une caisse enregistreuse, des bornes et une plateforme de vente en ligne. Nous avons délivré 22 millions de tickets grâce à des applications en monolithique et en microservices.

Je suis également impliqué(e) dans la plateforme de vente en ligne Ciné Boutique, où j'ai participé au démarrage, au développement, aux tests et à la maintenance. J'ai travaillé sur l'évolution de la plateforme qui a délivré un million de tickets en 3 ans, ainsi que sur l'intégration du parcours d'achat dans des CMS et le paiement avec TPE Hipay, Paypal et Paybox. J'ai également partiellement géré la montée en charge. Pour cela, j'ai utilisé les technologies Front-end Typescript/React, le rendu côté serveur et le Back-end NodeJs/MongoDB.

En outre, j'ai réalisé deux sites vitrine avec un CMS headless et un parcours d'achat en ligne en Typescript, React et NodeJs. J'ai automatisé le déploiement continu des services en développement grâce à Gitlab CI/CD et maintenu la documentation développeur à jour.

J'ai également réalisé une application de lecture de sous titres en utilisant le langage Flutter`,
            technologies: 'J\'ai utilisé Typescript, React, Jest, Webpack, Redux, Sass/Less, NodeJs, MongoDB, Postgresql, ExtJS, l\'ERP Dolibarr, le scheduler Rundeck; les TPE Hipay, Paypal, Paybox; Les outils GNU/Linux, Emacs, Docker compose, Gitlab, et le Gitlab CI/CD',
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
            tasks: 'Pendant ce projet, j\'ai implémenté une architecture MVC et un ORM dans une application.',
            technologies: 'Backend php, SQL',
        }]
    },
    skills: {
        software: [
            {
                icon: 'https://cdn.svgporn.com/logos/elm.svg',
                id: 1,
                level: 'J\'ai une expérience de quelques jours de pratique sur le langage Elm',
                name: 'Elm',
                technologies: [{
                    content: 'En utilisant Elm, j\'ai développé ce site et un [jeu du Pong](https://el-pong.netlify.app/) ',
                    id: 1,
                    name: 'Jeu/Front-end web'
                }]
            },
            {
                icon: 'https://upload.wikimedia.org/wikipedia/commons/a/ab/Cuddlyferris.svg',
                id: 2,
                level: 'J\'ai une expérience de quelques semaines de pratique sur le langage Rust',
                name: 'Rust',
                technologies: [{
                    content: 'En utilisant Rust, le framework Actix, React, Typescript et Elm, j\'ai eu l\'occation de développer le backend ainsi que le front end de ce site, et également d\'automatiser le déploiement en utilisant le Bitbucket CI/CD.',
                    id: 1,
                    name: 'Web'
                }, {
                    content: 'J\'ai eu l\'occasion de développer une [librairie](https://ctprods.cyprientaque.com/blog/a-rust-api-pattern-actix) pour le framework Actix',
                    id: 2,
                    name: 'Librairie'
                }]
            },
            {
                icon: 'https://cdn.svgporn.com/logos/typescript-icon.svg',
                id: 3,
                level: 'J\'ai une expérience de quelques années dans le langage Typescript et React',
                name: 'Typescript/React',
                technologies: [{
                    content: 'J\'ai eu l\'occation de travailler au démarrage, au développement, la maintenance et la refonte graphique de la plateforme de réservation de places de cinéma Ciné Boutique qui a délivré un million de tickets, par exemple sur le site (Exemple de [site](https://guerandecinepresquile.cine.boutique/)).',
                    id: 1,
                    name: 'Ciné Boutique',
                }, {
                    content: 'J\'ai eu l\'occation de travailler du démarrage à la livraison de deux sites vitrine et d\'un parcours d\'achat de billets de cinéma [www.cineum.fr](https://www.cineum.fr) (Cannes), [www.cineplanet.fr](https://www.cineplanet.fr)',
                    id: 2,
                    name: 'Cineum.fr - Cineplanet.fr'
                }, {
                    content: 'J\'ai eu l\'occation de travailler sur le démarrage d\'un projet d\'un tableau de bord d\'un système de gestion de cinéma (TMS, Theater Management System) en m\'interfaçant avec un backend en Java.',
                    id: 3,
                    name: 'Hyperviseur de TMS'
                }, {
                    content: 'J\'ai eu l\'occasion de participer au développement web et à la maintenance d\'un logiciel en ExtJS. (22 millions de tickets délivrés).',
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
                content: 'De l\'age de 6 ans à l\'âge de 18 ans j\'ai eu l\'occasion de pratiquer la voile en [compétition](http://www.ffvoile.fr/ffv/sportif/ClmtCoureurFiche.asp?clid=1057876E) de 2002 à 2008 en tant que membre de l\'équipe régionale, en utilisant les supports optimist, 420 puis à l\'age adulte en RS800 et 14 pieds international.',
                id: 1,
            }, {
                content: 'Pendant mon temps libre, j\'aime aller nager',
                id: 2,
            }]
        },
        {
            id: 2,
            name: 'Bricolage',
            content: [{
                content: 'Pendant mon temps libre j\'ai eu l\'occation de fabriquer des meubles et objets en bois et de rénover une fourgonette aménagée',
                id: 1,
            }]
        },
        {
            id: 3,
            name: 'Tourisme, voyages',
            content: [{
                content: 'J\'ai eu l\'occasion de voyager en Mer en bateau, à la montagne, au Canada et aux Etat Unis à l\'occasion d\'un stage de 5 mois en 2010, j\'ai également voyagé en Europe et en Afrique du nord. J\'apprécie de m\'avader dans la nature à l\'occasion d\'une bonne marche.',
                id: 1,
            }]
        }, {
            id: 4,
            name: 'Culture',
            content: [{
                content: 'J\'aime me plonger dans des livres portant sur les langages de développement, ainsi que des romans tels que ceux de John Irving et Jules Verne. J\'aime également lire sur des sujets variés, tels que la course à pied, ou encore des autobiographies, comme celle de Lisa Jobs.',
                id: 1
            }, {
                content: 'J\'apprécie de temps en temps un bon film, notamment ceux du cinéma d\'art et essai.',
                id: 2,
            }, {
                content: 'J\'aime écouter de la musique comme de la musique électronique minimaliste, de la musique classique, ou du folk.',
                id: 3,
            }]
        }
    ]

};

export default data;
