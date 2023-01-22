const data = {
    experiences: {
        software: [{
            beginning: '01/2018',
            contract: 'cdi',
            contract_type: 'travail',
            duration: 0,
            end: null,
            environment: 'SMEs, Software manufacturer',
            headline: 'Software developper',
            id: 1,
            institution: 'Ciné Digital',
            location: 'La Chapelle sur Erdre near Nantes',
            tasks: 'Web development mainly Typescript/React + Nodejs. Unit testing. Last project: [Ciné boutique](http://cdsdemorc1.cine.boutique/)',
            technologies: 'Typescript, React, Redux, Sass/Less, NodeJs, ExtJS',
        }, {
            beginning: '06/2017',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.35,
            end: '09/2017',
            environment: 'startup',
            headline: 'Full Stack developper',
            id: 2,
            institution: 'Asmoza',
            location: 'Nantes',
            tasks: 'Backend web developement',
            technologies: 'PHP, MySQL',
        }],
        wood: [{
            beginning: '02/2015',
            contract: 'intérim',
            contract_type: 'travail',
            duration: 1,
            end: '02/2016',
            environment: 'industrial (CA 36M€ - STX supplier)',
            headline: 'Product engineering',
            id: 1,
            institution: 'Altor Industrie',
            location: 'Clisson (44)',
            tasks: 'Design improvements and new products (folded sheet, polyester)',
            technologies: 'Solidworks 2014, EPDM, Cegid PMI',
        }, {
            beginning: '10/2013',
            contract: 'cdi',
            contract_type: 'travail',
            duration: 1,
            end: '10/2014',
            environment: 'industrial (25M€ turnover)',
            headline: 'Research and development product manager',
            id: 2,
            institution: 'Comec',
            location: 'La Tessoualle near Cholet',
            tasks: 'Woodwork design with accoustic, anti-burglar, fire performance. Measurement and analysis of manufacturing process time.',
            technologies: 'Topsolid V6',
        }, {
            beginning: '10/2011',
            contract: 'professionalisation',
            contract_type: 'alternance',
            duration: 1,
            end: '10/2012',
            environment: 'SMEs (1M€ turnover)',
            headline: 'Technicien bureau d’études par alternance',
            id: 3,
            institution: 'Euroformes',
            location: 'Guichen (35)',
            tasks: 'Conception de mobilier bois-Corian-métal-verre.',
            technologies: 'Rhinoceros, Solidworks, autocad',
        }, {
            beginning: '01/2011',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.4,
            end: '05/2011',
            environment: 'industriel',
            headline: 'BTS internship',
            id: 4,
            institution: 'Pasquet Menuiseries',
            location: 'near Rennes',
            tasks: 'Solution of a production hazard',
            technologies: 'Excel, Powerpoint',
        }, {
            beginning: '06/2010',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.3,
            end: '08/2010',
            environment: 'SMEs, small industry',
            headline: 'BTS internship in modular house industry.',
            id: 5,
            institution: 'Suprême Homes',
            location: 'Canada',
            tasks: 'Operator, solution of a production hazard',
            technologies: 'Excel',
        }, {
            beginning: '05/2009',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.2,
            end: '06/2009',
            environment: 'industriel',
            headline: 'intership in luxury layout for hosteling and vans',
            id: 6,
            institution: 'ST Bois Concept',
            location: 'La Chevrolière near Nantes',
            tasks: 'Manufacture of furniture prototypes',
            technologies: 'Woodworking machinery, power tools',
        }, {
            beginning: '12/2008',
            contract: 'stage',
            contract_type: 'stage',
            duration: 0.2,
            end: '01/2009',
            environment: 'industriel',
            headline: 'Internship at Bénéteau shipyard',
            id: 7,
            institution: 'Bénéteau shipyard',
            location: 'Le Poiré sur Vie (85)',
            tasks: 'Operator on an assembly line',
            technologies: 'Power tools',
        }],
    },
    home: {
        about: 'About',
        titleDev: 'Software development',
        software: {
            title: 'Software development',
            introduction: `Software developer at [Ciné digital services](https://www.cinedigitalservice.com/cds/), I have 4 years of practice in webapps development. I am currently doing [Ciné boutique](https://leplessisrobinsonmaisondesarts.cine.boutique/),
a webapp in React / Typescript / Redux, NodeJs and occasionaly doing a cinema cash register, a webapp in ExtJS and Java`
        },
        wood: {
            title: 'Wood manufacturing and CAD engineering',
            introduction: `In another life...
I studied wood processing and CAD drawing. I have the basics of manual wood working, the use of woodworking machines, the theory of industrial productique and the practice of drawind and CAD and engineering. I did the job of technician (drawer, R&D project manager) in domains like industrial joinery, furniture layout, engineering of bathrooms in composite materials and folded sheet. See my [portfolio](https://www.cyprientaque.com/portfolio) for examples of project I did in this domain.`
        }
    },
    skills: {
        software: [
            {
                id: 1,
                icon: 'https://cdn.svgporn.com/logos/elm.svg',
                level: 'Days of practice',
                name: 'Elm',
                technologies: [{
                    content: 'Components development and a [Pong](https://el-pong.netlify.app/) game',
                    id: 1,
                    name: 'Game/Front end components'
                }]
            },  
            {
                icon: 'https://upload.wikimedia.org/wikipedia/commons/a/ab/Cuddlyferris.svg',
                id: 2,
                level: 'Weeks of practice',
                name: 'Rust',
                technologies: [{
                    content: 'Backend development of cyprientaque.com',
                    id: 1,
                    name: 'Web backend'
                }, {
                    content: 'Development of a Rest Api [library](https://ctprods.cyprientaque.com/blog/a-rust-api-pattern-actix) for the framework Actix',
                    id: 2,
                    name: 'Lib'
                }]
            },
            {
                id: 3,
                level: 'Years of practice',
                name: 'Typescript/Javascript',
                icon: 'https://cdn.svgporn.com/logos/typescript-icon.svg',
                technologies: [, {
                    content: 'Development, maintenance of a cinema tickets distance selling platform.',
                    id: 1,
                    name: 'Typescript, React, Redux, Jest, NodeJS, MongoDB',
                },{
                    content: 'Development, maintenance of two static websites with dynamic content and a sale course [www.cineum.fr](https://www.cineum.fr) (Cannes), [www.cineplanet.fr](http://www.cineplanet.fr).',
                    id: 2,
                    name: 'CMS Strapi, nodejs, Typescript, React'
                }, {
                    content: 'Development of an "hypervisor" (dashboard, hypervision) of a cinema management software (TMS, Theater Management System). Interfacing with a Java backend.',
                    id: 3,
                    name: 'Typescript/React/Redux/Redux Toolkit/React Query, Scss'
                }, {
                    content: 'Development and maintenance of a cinema cash register software.',
                    id: 4,
                    name: 'ExtJs',
                }]
            }
        ],
        wood: [{
            id: 2,
            level: '3 years of professional practice',
            name: 'CAO-DAO',
            technologies: [{
                id: 1,
                level: '',
                name: 'Software: Solidworks, Topsolid, Epdm, Rhinoceros, Inventor',
            }]
        }]
    },
    studies: {
        software: [{
            beginning: '11/2016',
            duration: 0.8,
            end: '09/2017',
            icon: 'https://s3-eu-west-1.amazonaws.com/ctaque.logos/imie.png',
            id: 1,
            institution: 'IMIE',
            location: 'Nantes',
            text: 'Professional title software developper - French RNCP level 3 (Bachelor+2y)',
        }],
        wood: [{
            beginning: '10/2011',
            duration: 1,
            end: '10/2012',
            icon: 'https://s3-eu-west-1.amazonaws.com/ctaque.logos/cci.jpg',
            id: 1,
            institution: 'Chamber of commerce and industry',
            location: 'Lorient',
            text: 'Qualification certificate specialized in CAD',
        }, {
            beginning: '09/2009',
            duration: 2,
            end: '06/2011',
            icon: 'https://s3-eu-west-1.amazonaws.com/ctaque.logos/esb.png',
            id: 2,
            institution: 'Wood high school',
            location: 'Nantes',
            text: 'BTS Productive Wood Option Management of Industrial Production',

        }]
    },
    hobbies: [
        {
            id: 1,
            name: 'Sport',
            content: [{
                id: 1,
                content: 'Sailing in [competition](http://www.ffvoile.fr/ffv/sportif/ClmtCoureurFiche.asp?clid=1057876E) from 2002 to 2008 as regional team member (dinghy).'
            }, {
                id: 2,
                content: 'Swimming, mountain walk..'
            }]
        },
        {
            id: 2,
            name: 'DIY',
            content: [{
                id: 1,
                content: 'Creation of furniture and wooden objects.'
            }, {
                id: 2,
                content: 'Renovation of a van : eletricity, furniture.'
            }]
        },
        {
            id: 3,
            name: 'Tourism, traveling',
            content: [{
                id: 1,
                content: 'Seaside, mountain, Canada (5 months internship in 2010), USA, Europe, North Africa... I like nature and [photography](https://www.flickr.com/people/114643587@N06/).'
            }]
        }, {
            id: 4,
            name: 'Culture',
            content: [{
                content: 'Reading: Books of comp Sci, Blogs, Reddit posts and autobiographies',
                id: 1
            }, {
                content: 'Cinéma: cinéma "d\'art et essai", (Love it !)',
                id: 2,
            }, {
                content: 'Music: Electronic minimalist, classical, folk, and some TECHNO',
                id: 3,
            }]
        }
    ]

};

export default data;
