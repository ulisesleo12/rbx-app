const {exec} = require("child_process");
const { log } = require("console");
const fs = require('fs');
const XLSX = require('xlsx');

var xlsx = require('node-xlsx');

var data = xlsx.parse(fs.readFileSync(__dirname + '/COLEGIO-CRISTOBAL-COLON.xlsx')); 

// console.log(data[0].data.shift());
data[0].data.shift();

// console.log(data[0]['data']);

function getGroupId(code, section) {
    let group_id = "";
    switch(code) {
        case "K4":
            switch(section) {
                case "A": 
                    group_id = "2a7ccb13-d89b-4987-93d5-7d7669147edf";
                    break;
                case "B": 
                    group_id = "2b4dce57-5b62-42a1-a82f-5d90f30da7eb";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "K5":
            switch(section) {
                case "A": 
                    group_id = "ae8b21df-ec43-4370-9fe6-53b60b889144";
                    break;
                case "B": 
                    group_id = "dd449a1c-4f0b-4779-be95-7932ee67832a";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "PP":
            switch(section) {
                case "A": 
                    group_id = "bff28d59-8d1d-4e40-80b1-f5fca7601468";
                    break;
                case "B": 
                    group_id = "7e377564-f4e3-4f66-bdce-10783c4b5fc7";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P1":
            switch(section) {
                case "A": 
                    group_id = "2432247b-0c1b-4eb0-a7d7-c0658b440670";
                    break;
                case "B": 
                    group_id = "e9685372-6c7b-4807-9550-3bdcdd4cd7f5";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P2":
            switch(section) {
                case "A": 
                    group_id = "8db24d62-0b7e-44f1-8ad4-11f3a2c2f5fe";
                    break;
                case "B": 
                    group_id = "641d5f01-f6c8-4da1-80a6-ede4647397a8";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P3":
            switch(section) {
                case "A": 
                    group_id = "c4a3c5bd-0f06-4afd-8fcf-f24d935c7ac6";
                    break;
                case "B": 
                    group_id = "0514e05c-792b-42cd-b479-5eb0b3e5beca";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P4":
            switch(section) {
                case "A": 
                    group_id = "8f2918d7-00db-4d79-992a-d6f28bb5373c";
                    break;
                case "B": 
                    group_id = "e9e4de09-0cb9-46cf-9e35-3e914eb3d755";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P5":
            switch(section) {
                case "A": 
                    group_id = "ff12b194-87a7-4ea2-8dc6-51a559f00ada";
                    break;
                case "B": 
                    group_id = "d3e8f40d-75fe-40e8-a689-efd807b10d93";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P6":
            switch(section) {
                case "A": 
                    group_id = "11f9c88e-dbd1-437d-92c1-83f8f95be788";
                    break;
                case "B": 
                    group_id = "495a9217-89ce-4b58-9dab-5643c631b23d";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "S1":
            switch(section) {
                case "A": 
                    group_id = "7ba6b3f9-9157-403a-b38e-1a4235e69c9f";
                    break;
                case "B": 
                    group_id = "1b2946eb-0a0a-420a-a9ca-8ff68f0bd541";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "S2":
            switch(section) {
                case "A": 
                    group_id = "107f23ee-8d68-4649-ac06-2551d27a2ae5";
                    break;
                case "B": 
                    group_id = "c4ce908b-698e-4761-bf4d-e9f185aac785";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "S3":
            switch(section) {
                case "A": 
                    group_id = "e3694aa5-9454-4931-9880-d28f7da6b199";
                    break;
                case "B": 
                    group_id = "322ddec0-f726-4d4e-8cac-206a1d765a52";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "B1":
            switch(section) {
                case "A": 
                    group_id = "392ec029-add3-4b64-ab44-a5ac8fb6f7c4";
                    break;
                case "B": 
                    group_id = "b19b1c84-2636-40df-a6f1-f06fba87f096";
                    break;
                case "C": 
                    group_id = "7f29fcde-6dee-46e7-9120-2ddbced6c19e";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "B2":
            switch(section) {
                case "A": 
                    group_id = "c836f443-122c-4eb3-9d9d-5139b484646c";
                    break;
                case "B": 
                    group_id = "0de97987-aecc-4ead-9833-c157baf72a90";
                    break;
                default:
                    group_id = "";
            }
            break;
        default:
            group_id = "";
    }
    return group_id;
}

user_exists = [
    "edgardo.zaldivar@colegiocristobalcolon.edu.sv",
    "joyce.sanchez@colegiocristobalcolon.edu.sv",
    "eduardo.sanabria@colegiocristobalcolon.edu.sv",
    "adriana.melara@colegiocristobalcolon.edu.sv",
    "edgard.gonzalez@colegiocristobalcolon.edu.sv",
    "milena.palacios@colegiocristobalcolon.edu.sv",
    "sofia.alvarenga@colegiocristobalcolon.edu.sv",
    "matilda.cerna@colegiocristobalcolon.edu.sv",
    "ahmed.menjivar@colegiocristobalcolon.edu.sv",
    "paula.castro@colegiocristobalcolon.edu.sv",
    "camila.castro@colegiocristobalcolon.edu.sv",
    "christian.barrera@colegiocristobalcolon.edu.sv",
    "andrea.rivera@colegiocristobalcolon.edu.sv",
    "nahomy.larin@colegiocristobalcolon.edu.sv",
    "pablo.valdes@colegiocristobalcolon.edu.sv",
    "daniela.rodriguez@colegiocristobalcolon.edu.sv",
    "renata.valdes@colegiocristobalcolon.edu.sv",
    "ambar.sanchez@colegiocristobalcolon.edu.sv",
    "rodrigo.bonilla@colegiocristobalcolon.edu.sv",
    "natalia.melchor@colegiocristobalcolon.edu.sv",
    "daniel.melchor@colegiocristobalcolon.edu.sv",
    "diego.quintanilla@colegiocristobalcolon.edu.sv",
    "santiago.beltran@colegiocristobalcolon.edu.sv",
    "daniela.reyes01@colegiocristobalcolon.edu.sv",
    "daniel.barrios@colegiocristobalcolon.edu.sv",
    "mateo.barrios@colegiocristobalcolon.edu.sv",
    "susana.lainez@colegiocristobalcolon.edu.sv",
    "roberto.marquez@colegiocristobalcolon.edu.sv",
    "valeria.gavidia@colegiocristobalcolon.edu.sv",
    "adriana.gavidia@colegiocristobalcolon.edu.sv",
    "andrea.canjura@colegiocristobalcolon.edu.sv",
    "luis.flores@colegiocristobalcolon.edu.sv",
    "mario.merlos01@colegiocristobalcolon.edu.sv",
    "mateo.campos@colegiocristobalcolon.edu.sv",
    "antonio.rodriguez@colegiocristobalcolon.edu.sv",
    "ricardo.rodriguez@colegiocristobalcolon.edu.sv",
    "renato.moran@colegiocristobalcolon.edu.sv",
    "david.martinez@colegiocristobalcolon.edu.sv",
    "leandra.cobar@colegiocristobalcolon.edu.sv",
    "juan.lopez@colegiocristobalcolon.edu.sv",
    "fabian.varela@colegiocristobalcolon.edu.sv",
    "norman.orantes@colegiocristobalcolon.edu.sv",
    "mateo.garcia@colegiocristobalcolon.edu.sv",
    "fabricio.calderon@colegiocristobalcolon.edu.sv",
    "rodrigo.rivas@colegiocristobalcolon.edu.sv",
    "sebastian.jovel@colegiocristobalcolon.edu.sv",
    "rene.henriquez01@colegiocristobalcolon.edu.sv",
    "nelson.rosales@colegiocristobalcolon.edu.sv",
    "joseph.fuentes@colegiocristobalcolon.edu.sv",
    "sofia.bonilla@colegiocristobalcolon.edu.sv",
    "diego.barrera@colegiocristobalcolon.edu.sv",
    "amalia.pacheco@colegiocristobalcolon.edu.sv",
    "leonardohong.yushen@colegiocristobalcolon.edu.sv",
    "derek.soria@colegiocristobalcolon.edu.sv",
    "grace.jacinto@colegiocristobalcolon.edu.sv",
    "andrea.arteaga@colegiocristobalcolon.edu.sv",
    "carlos.hernandez@colegiocristobalcolon.edu.sv",
    "matias.alfonso01@colegiocristobalcolon.edu.sv",
    "karla.posada@colegiocristobalcolon.edu.sv",
    "allison.aquino01@colegiocristobalcolon.edu.sv",
    "cristina.galdamez@colegiocristobalcolon.edu.sv",
    "aida.zetino@colegiocristobalcolon.edu.sv",
    "julio.rodas@colegiocristobalcolon.edu.sv",
    "monica.jandres@colegiocristobalcolon.edu.sv",
    "carlos.jovel@colegiocristobalcolon.edu.sv",
    "rocio.morales@colegiocristobalcolon.edu.sv",
    "fernando.castillo@colegiocristobalcolon.edu.sv",
    "alessa.cuellar@colegiocristobalcolon.edu.sv",
    "joseline.sanchez01@colegiocristobalcolon.edu.sv",
    "sofia.cortez@colegiocristobalcolon.edu.sv",
    "jose.ramos01@colegiocristobalcolon.edu.sv",
    "cesar.alvarado@colegiocristobalcolon.edu.sv",
    "gabriela.ortega01@colegiocristobalcolon.edu.sv",
    "andrea.anaya@colegiocristobalcolon.edu.sv",
    "melvin.gomez@colegiocristobalcolon.edu.sv",
    "gabriel.espinoza@colegiocristobalcolon.edu.sv",
    "maya.alvarado@colegiocristobalcolon.edu.sv",
    "fernando.pacas@colegiocristobalcolon.edu.sv",
    "hector.alvarado01@colegiocristobalcolon.edu.sv",
    "gabriel.gonzalez@colegiocristobalcolon.edu.sv",
    "tatiana.alfonso@colegiocristobalcolon.edu.sv",
    "bryan.navarro@colegiocristobalcolon.edu.sv",
    "alison.duran@colegiocristobalcolon.edu.sv",
    "armando.marti@colegiocristobalcolon.edu.sv",
    "jefferson.marroquin@colegiocristobalcolon.edu.sv",
    "jose.castro@colegiocristobalcolon.edu.sv",
    "hanna.martin@colegiocristobalcolon.edu.sv",
    "fernando.cortez@colegiocristobalcolon.edu.sv",
    "sara.guevara@colegiocristobalcolon.edu.sv",
    "jabes.lainez@colegiocristobalcolon.edu.sv",
    "karla.munoz01@colegiocristobalcolon.edu.sv",
    "dalila.montenegro@colegiocristobalcolon.edu.sv",
    "daveyda.fuentes@colegiocristobalcolon.edu.sv",
    "maria.escobar@colegiocristobalcolon.edu.sv",
    "pamela.ayala@colegiocristobalcolon.edu.sv",
    "nelson.portillo@colegiocristobalcolon.edu.sv",
    "adrian.amaya@colegiocristobalcolon.edu.sv",
    "diego.mancia01@colegiocristobalcolon.edu.sv",
    "esthela.villalpando@colegiocristobalcolon.edu.sv",
    "rafael.contreras@colegiocristobalcolon.edu.sv",
    "henry.molina@colegiocristobalcolon.edu.sv",
    "emely.belen@colegiocristobalcolon.edu.sv",
    "brandon.lazo@colegiocristobalcolon.edu.sv",
    "mauricio.garcia@colegiocristobalcolon.edu.sv",
    "luis.martinez01@colegiocristobalcolon.edu.sv",
    "leonardo.flores@colegiocristobalcolon.edu.sv",
    "nicole.martinez@colegiocristobalcolon.edu.sv",
    "angel.depaz@colegiocristobalcolon.edu.sv",
    "mathew.melendez@colegiocristobalcolon.edu.sv",
    "mariana.guzman01@colegiocristobalcolon.edu.sv",
    "camila.sanchez@colegiocristobalcolon.edu.sv",
    "edwin.flores@colegiocristobalcolon.edu.sv",
    "samuel.hernandez01@colegiocristobalcolon.edu.sv",
    "cristian.alonso@colegiocristobalcolon.edu.sv",
    "roberto.minero@colegiocristobalcolon.edu.sv",
    "america.martinez@colegiocristobalcolon.edu.sv",
    "maritza.mejia@colegiocristobalcolon.edu.sv",
    "gabriel.lemus@colegiocristobalcolon.edu.sv",
    "adriana.cuellar@colegiocristobalcolon.edu.sv",
    "katherine.morales@colegiocristobalcolon.edu.sv",
    "kevin.hernandez@colegiocristobalcolon.edu.sv",
    "daniela.maldonado@colegiocristobalcolon.edu.sv",
    "grecia.lopez@colegiocristobalcolon.edu.sv",
    "allison.perez@colegiocristobalcolon.edu.sv",
    "gustavo.rosales@colegiocristobalcolon.edu.sv",
    "carlos.garcia@colegiocristobalcolon.edu.sv",
    "claudia.escoto@colegiocristobalcolon.edu.sv",
    "angello.godinez@colegiocristobalcolon.edu.sv",
    "maria.cea01@colegiocristobalcolon.edu.sv",
    "karla.ayala@colegiocristobalcolon.edu.sv",
    "andrea.chavarria@colegiocristobalcolon.edu.sv",
    "tiffany.coreas@colegiocristobalcolon.edu.sv",
    "rodolfo.gomez@colegiocristobalcolon.edu.sv",
    "andrea.gonzalez@colegiocristobalcolon.edu.sv",
    "paola.martinez@colegiocristobalcolon.edu.sv",
    "alfonso.gonzalez@colegiocristobalcolon.edu.sv",
    "elsy.hernandez@colegiocristobalcolon.edu.sv",
    "jefferson.hernandez@colegiocristobalcolon.edu.sv",
    "valeria.mendez@colegiocristobalcolon.edu.sv",
    "ivan.figueroa@colegiocristobalcolon.edu.sv",
    "marcela.henriquez@colegiocristobalcolon.edu.sv",
    "ricardo.martinez@colegiocristobalcolon.edu.sv",
    "gianna.torres@colegiocristobalcolon.edu.sv",
    "rodrigo.cortez@colegiocristobalcolon.edu.sv",
    "ariana.saravia@colegiocristobalcolon.edu.sv",
    "ricardo.gonzalez@colegiocristobalcolon.edu.sv",
    "sofia.hernandez01@colegiocristobalcolon.edu.sv",
    "lourdes.marin@colegiocristobalcolon.edu.sv",
    "adonis.melgar@colegiocristobalcolon.edu.sv",
    "salvador.molina@colegiocristobalcolon.edu.sv",
    "nathaly.sotelo@colegiocristobalcolon.edu.sv",
    "francisco.ramos@colegiocristobalcolon.edu.sv",
    "camila.portillo@colegiocristobalcolon.edu.sv",
    "luis.hernandez01@colegiocristobalcolon.edu.sv",
    "belinda.martinez@colegiocristobalcolon.edu.sv",
    "anthony.cerna01@colegiocristobalcolon.edu.sv",
    "joaquin.melhado@colegiocristobalcolon.edu.sv",
    "jorge.quinteros@colegiocristobalcolon.edu.sv",
    "jorge.deras@colegiocristobalcolon.edu.sv",
    "dennis.cruz01@colegiocristobalcolon.edu.sv",
    "valeria.vigil@colegiocristobalcolon.edu.sv",
    "johanna.lainez@colegiocristobalcolon.edu.sv",
    "diego.zavaleta@colegiocristobalcolon.edu.sv",
    "raul.chavez@colegiocristobalcolon.edu.sv",
    "farah.molina@colegiocristobalcolon.edu.sv",
    "alessandro.ruiz@colegiocristobalcolon.edu.sv",
    "angie.perez@colegiocristobalcolon.edu.sv",
    "andrea.olivar@colegiocristobalcolon.edu.sv",
    "nestor.rivas@colegiocristobalcolon.edu.sv",
    "andrea.montes@colegiocristobalcolon.edu.sv",
    "josue.rodriguez@colegiocristobalcolon.edu.sv",
    "victor.morales@colegiocristobalcolon.edu.sv",
    "ricardo.ruano@colegiocristobalcolon.edu.sv",
    "maria.rosales@colegiocristobalcolon.edu.sv",
    "gabriel.tobar@colegiocristobalcolon.edu.sv",
    "mario.salvador@colegiocristobalcolon.edu.sv",
    "katherine.ruano@colegiocristobalcolon.edu.sv",
    "fernando.palucho@colegiocristobalcolon.edu.sv",
    "tiphanie.serpas@colegiocristobalcolon.edu.sv",
    "adriana.reyes@colegiocristobalcolon.edu.sv",
    "leonardo.romero01@colegiocristobalcolon.edu.s",
    "cristian.pacheco01@colegiocristobalcolon.edu.sv",
    "veronica.munoz01@colegiocristobalcolon.edu.sv",
    "rodrigo.rivas01@colegiocristobalcolon.edu.sv",
    "daniela.rivas@colegiocristobalcolon.edu.sv",
    "jorge.gonzalez@colegiocristobalcolon.edu.sv",
    "fabiola.rivera@colegiocristobalcolon.edu.sv",
    "felipe.cerna@colegiocristobalcolon.edu.sv",
    "mario.valle@colegiocristobalcolon.edu.sv",
    "carlos.moz@colegiocristobalcolon.edu.sv",
    "valeria.munoz@colegiocristobalcolon.edu.sv",
    "helena.escobar@colegiocristobalcolon.edu.sv",
    "lizbeth.morataya01@colegiocristobalcolon.edu.sv",
    "diego.ramirez@colegiocristobalcolon.edu.sv",
    "emma.aguilar@colegiocristobalcolon.edu.sv",
    "jose.gomez@colegiocristobalcolon.edu.sv",
    "andrea.palma@colegiocristobalcolon.edu.sv",
    "yuliam.pineda@colegiocristobalcolon.edu.sv",
    "dania.alvarenga@colegiocristobalcolon.edu.sv",
    "daniela.vasconcelos@colegiocristobalcolon.edu.sv",
    "fernando.ulloa@colegiocristobalcolon.edu.sv",
    "jose.olmedo@colegiocristobalcolon.edu.sv",
    "elena.pacheco01@colegiocristobalcolon.edu.sv",
    "santiago.reyes@colegiocristobalcolon.edu.sv",
    "diego.rivera01@colegiocristobalcolon.edu.sv",
    "emely.arevalo@colegiocristobalcolon.edu.sv",
    "fernando.ramirez@colegiocristobalcolon.edu.sv",
    "victoria.polanco@colegiocristobalcolon.edu.sv",
    "sebastian.bou@colegiocristobalcolon.edu.sv",
    "paola.montenegro@colegiocristobalcolon.edu.sv",
    "mario.sermeno@colegiocristobalcolon.edu.sv",
    "leonardo.carranza@colegiocristobalcolon.edu.sv",
    "william.rodriguez@colegiocristobalcolon.edu.sv",
    "ambar.juarez@colegiocristobalcolon.edu.sv",
    "ruby.hernandez@colegiocristobalcolon.edu.sv",
    "liam.salguero@colegiocristobalcolon.edu.sv",
    "isabella.martinez@colegiocristobalcolon.edu.sv",
    "patricia.reyes@colegiocristobalcolon.edu.sv",
    "maria.morales@colegiocristobalcolon.edu.sv",
    "diana.cortez@colegiocristobalcolon.edu.sv",
    "ricardo.mendez@colegiocristobalcolon.edu.sv",
    "duvan.moreno@colegiocristobalcolon.edu.sv",
    "cesar.ramirez@colegiocristobalcolon.edu.sv",
    "miguel.guzman@colegiocristobalcolon.edu.sv",
    "giuliana.polanco@colegiocristobalcolon.edu.sv",
    "oscar.escalante@colegiocristobalcolon.edu.sv",
    "miguel.alberto@colegiocristobalcolon.edu.sv",
    "lucas.rodriguez@colegiocristobalcolon.edu.sv",
    "diego.alfaro@colegiocristobalcolon.edu.sv",
    "renata.osorio@colegiocristobalcolon.edu.sv",
    "alexa.dominguez@colegiocristobalcolon.edu.sv",
    "stefany.iraheta@colegiocristobalcolon.edu.sv",
    "maria.monge@colegiocristobalcolon.edu.sv",
    "ian.pacheco@colegiocristobalcolon.edu.sv",
    "augusto.salinas@colegiocristobalcolon.edu.sv",
    "josue.aguilar@colegiocristobalcolon.edu.sv",
    "ariana.palacios@colegiocristobalcolon.edu.sv",
    "ariana.velasquez@colegiocristobalcolon.edu.sv",
    "darian.garcia@colegiocristobalcolon.edu.sv",
    "regina.cubias@colegiocristobalcolon.edu.sv",
    "emely.marmol@colegiocristobalcolon.edu.sv",
    "thais.hernandez@colegiocristobalcolon.edu.sv",
    "ximena.rivera@colegiocristobalcolon.edu.sv",
    "maria.marroquin@colegiocristobalcolon.edu.sv",
    "juliana.rodas@colegiocristobalcolon.edu.sv",
    "lester.jerez@colegiocristobalcolon.edu.sv",
    "darwin.velasquez@colegiocristobalcolon.edu.sv",
    "luis.villacorta01@colegiocristobalcolon.edu.sv",
    "lucas.guandique@colegiocristobalcolon.edu.sv",
    "alicia.hernandez@colegiocristobalcolon.edu.sv",
    "fatima.guidos@colegiocristobalcolon.edu.sv",
    "jonathan.mercadillo@colegiocristobalcolon.edu.sv",
    "stephanie.maldonado@colegiocristobalcolon.edu.sv",
    "rolando.carrero@colegiocristobalcolon.edu.sv",
    "luis.candray@colegiocristobalcolon.edu.sv",
    "mateo.hernandez@colegiocristobalcolon.edu.sv",
    "john.avalos01@colegiocristobalcolon.edu.sv",
    "ariadna.ayala@colegiocristobalcolon.edu.sv",
    "santiago.vides@colegiocristobalcolon.edu.sv",
    "ariadna.perez@colegiocristobalcolon.edu.sv",
    "santiago.flores@colegiocristobalcolon.edu.sv",
    "marcela.guzman@colegiocristobalcolon.edu.sv",
    "gabriel.rodas@colegiocristobalcolon.edu.sv",
    "lucas.molina@colegiocristobalcolon.edu.sv",
    "marcos.chicas@colegiocristobalcolon.edu.sv",
    "sofia.orantes@colegiocristobalcolon.edu.sv",
    "alberto.valladares@colegiocristobalcolon.edu.sv",
    "benjamin.rodriguez@colegiocristobalcolon.edu.sv",
    "mariana.montenegro@colegiocristobalcolon.edu.sv",
    "isabella.lopez@colegiocristobalcolon.edu.sv",
    "victor.lainez@colegiocristobalcolon.edu.sv",
    "gabriela.carbajal@colegiocristobalcolon.edu.sv",
    "keilyn.rivera@colegiocristobalcolon.edu.sv",
    "daniella.sosa@colegiocristobalcolon.edu.sv",
    "maia.herrera@colegiocristobalcolon.edu.sv",
    "matias.iraheta@colegiocristobalcolon.edu.sv",
    "shirley.lopez@colegiocristobalcolon.edu.sv",
    "antony.molina@colegiocristobalcolon.edu.sv",
    "marjorie.aquino01@colegiocristobalcolon.edu.sv",
    "carlos.ayala@colegiocristobalcolon.edu.sv",
    "adelaida.ticas@colegiocristobalcolon.edu.sv",
    "stephannie.sanchez01@colegiocristobalcolon.edu.sv",
    "marcos.ardon@colegiocristobalcolon.edu.sv",
    "derek.larin@colegiocristobalcolon.edu.sv",
    "derek.espinoza@colegiocristobalcolon.edu.sv",
    "rommel.escobar@colegiocristobalcolon.edu.sv",
    "carlos.romero@colegiocristobalcolon.edu.sv",
    "adrian.ramos@colegiocristobalcolon.edu.sv",
    "aaron.quintanilla@colegiocristobalcolon.edu.sv",
    "sayra.granadeno@colegiocristobalcolon.edu.sv",
    "david.montoya01@colegiocristobalcolon.edu.sv",
    "ana.ortiz@colegiocristobalcolon.edu.sv",
    "melvin.reyes@colegiocristobalcolon.edu.sv",
    "zoe.melgar@colegiocristobalcolon.edu.sv",
    "lenny.alfaro@colegiocristobalcolon.edu.sv",
    "gerardo.arteaga@colegiocristobalcolon.edu.sv",
    "izel.martínez@colegiocristobalcolon.edu.sv",
    "esteban.valdés@colegiocristobalcolon.edu.sv",
    "mateo.montenegro@colegiocristobalcolon.edu.sv",
    "alex.bermúdez@colegiocristobalcolon.edu.sv",
    "estrella.cardoza@colegiocristobalcolon.edu.sv",
    "matthew.garcia@colegiocristobalcolon.edu.sv",
    "rocio.garcia@colegiocristobalcolon.edu.sv",
    "santiago.batres@colegiocristobalcolon.edu.sv",
    "valentina.castro@colegiocristobalcolon.edu.sv",
    "valeria.zometa@colegiocristobalcolon.edu.sv",
    "giselle.alvarado@colegiocristobalcolon.edu.sv",
    "frida.castro@colegiocristobalcolon.edu.sv",
    "fernando.rivas@colegiocristobalcolon.edu.sv",
    "emma.merino@colegiocristobalcolon.edu.sv",
    "maria.leon@colegiocristobalcolon.edu.sv",
    "amelie.aguilar@colegiocristobalcolon.edu.sv",
    "kimberly.ortiz@colegiocristobalcolon.edu.sv",
    "mayte.lopez@colegiocristobalcolon.edu.sv",
    "valeria.chicas@colegiocristobalcolon.edu.sv",
    "josseline.portillo@colegiocristobalcolon.edu.sv",
    "damaris.zuniga@colegiocristobalcolon.edu.sv",
    "maria.aguilar@colegiocristobalcolon.edu.sv",
    "leandro.rosales@colegiocristobalcolon.edu.sv",
    "valeria.zamora@colegiocristobalcolon.edu.sv",
    "axel.rivas@colegiocristobalcolon.edu.sv",
    "douglas.montes@colegiocristobalcolon.edu.sv",
    "elias.rosales@colegiocristobalcolon.edu.sv",
    "marcelo.alvarado@colegiocristobalcolon.edu.sv",
    "andres.zamora@colegiocristobalcolon.edu.sv",
    "sofia.montano@colegiocristobalcolon.edu.sv",
    "dominic.berdugo@colegiocristobalcolon.edu.sv",
    "fiorela.alberto01@colegiocristobalcolon.edu.sv",
    "montserrat.martinez@colegiocristobalcolon.edu.sv",
    "gabriela.aguilar@colegiocristobalcolon.edu.sv",
    "esteban.argueta@colegiocristobalcolon.edu.sv",
    "carlos.gomez@colegiocristobalcolon.edu.sv",
    "leonardo.alvarenga@colegiocristobalcolon.edu.sv",
    "fabiola.aguirre@colegiocristobalcolon.edu.sv",
    "zoe.lopez@colegiocristobalcolon.edu.sv",
    "demian.cordero@colegiocristobalcolon.edu.sv",
    "sebastian.murcia@colegiocristobalcolon.edu.sv",
    "mia.castellon@colegiocristobalcolon.edu.sv",
    "paolo.martinez@colegiocristobalcolon.edu.sv",
    "valeria.guevara@colegiocristobalcolon.edu.sv",
    "zara.alvarez@colegiocristobalcolon.edu.sv",
    "jakel.hernandez@colegiocristobalcolon.edu.sv",
    "victor.sanchez@colegiocristobalcolon.edu.sv",
    "santiago.valencia@colegiocristobalcolon.edu.sv",
    "liam.gonzalez@colegiocristobalcolon.edu.sv",
    "eduardo.arevalo@colegiocristobalcolon.edu.sv",
    "montserrat.martinez01@colegiocristobalcolon.edu.sv",
    "javier.tario@colegiocristobalcolon.edu.sv",
    "ian.aleman@colegiocristobalcolon.edu.sv",
    "matteo.hernandez@colegiocristobalcolon.edu.sv",
    "elena.zavaleta@colegiocristobalcolon.edu.sv",
    "sarah.garcia@colegiocristobalcolon.edu.sv",
    "herson.carcamo@colegiocristobalcolon.edu.sv",
    "carlos.fajardo@colegiocristobalcolon.edu.sv",
    "mateo.castro@colegiocristobalcolon.edu.sv",
    "donovan.moreno@colegiocristobalcolon.edu.sv",
    "axel.rodriguez@colegiocristobalcolon.edu.sv",
    "sofia.serrano@colegiocristobalcolon.edu.sv",
    "santiago.rodriguez@colegiocristobalcolon.edu.sv",
    "francisco.cardoza@colegiocristobalcolon.edu.sv",
    "fernando.canton@colegiocristobalcolon.edu.sv",
    "raul.romero@colegiocristobalcolon.edu.sv",
    "gabriela.ayala@colegiocristobalcolon.edu.sv",
    "dayana.figueroa@colegiocristobalcolon.edu.sv",
    "noah.cortez@colegiocristobalcolon.edu.sv",
    "isabella.guerrero@colegiocristobalcolon.edu.sv",
    "elvin.jerez01@colegiocristobalcolon.edu.sv",
    "mauro.moreno@colegiocristobalcolon.edu.sv",
    "oscar.hernandez@colegiocristobalcolon.edu.sv",
    "carlos.vides@colegiocristobalcolon.edu.sv",
    "diego.sandoval@colegiocristobalcolon.edu.sv",
    "manuel.kessels@colegiocristobalcolon.edu.sv",
    "dennys.aguilar@colegiocristobalcolon.edu.sv",
    "julieta.jacobo@colegiocristobalcolon.edu.sv",
    "fernando.chicas@colegiocristobalcolon.edu.sv",
    "megan.hernandez@colegiocristobalcolon.edu.sv",
    "ximena.penate@colegiocristobalcolon.edu.sv",
    "santiago.mejia@colegiocristobalcolon.edu.sv",
    "nelly.palacios@colegiocristobalcolon.edu.sv",
    "rodrigo.rosales@colegiocristobalcolon.edu.sv",
    "aylin.ortiz@colegiocristobalcolon.edu.sv",
    "valeria.romero@colegiocristobalcolon.edu.sv",
    "emily.castro@colegiocristobalcolon.edu.sv",
    "genesis.artiga@colegiocristobalcolon.edu.sv",
    "celina.vides@colegiocristobalcolon.edu.sv",
    "cesar.duenas@colegiocristobalcolon.edu.sv",
    "victor.mira@colegiocristobalcolon.edu.sv",
    "valentina.herrera01@colegiocristobalcolon.edu.sv",
    "eduardo.castellon@colegiocristobalcolon.edu.sv",
    "axel.barahona@colegiocristobalcolon.edu.sv",
    "alisson.alvarez@colegiocristobalcolon.edu.sv",
    "diego.reyes@colegiocristobalcolon.edu.sv",
    "joel.alas@colegiocristobalcolon.edu.sv",
    "fabiola.zuniga@colegiocristobalcolon.edu.sv",
    "jose.sanchez@colegiocristobalcolon.edu.sv",
    "mateo.menjivar@colegiocristobalcolon.edu.sv",
    "alicia.guevara01@colegiocristobalcolon.edu.sv",
    "erick.ramirez01@colegiocristobalcolon.edu.sv",
    "fernanda.sandoval@colegiocristobalcolon.edu.sv",
    "angel.siguenza@colegiocristobalcolon.edu.sv",
    "samantha.lopez01@colegiocristobalcolon.edu.sv",
    "ricardo.guerra@colegiocristobalcolon.edu.sv",
    "alejandro.argueta@colegiocristobalcolon.edu.sv",
    "gabriela.hernandez01@colegiocristobalcolon.edu.sv",
    "irene.garay@colegiocristobalcolon.edu.sv",
    "jose.rodriguez@colegiocristobalcolon.edu.sv",
    "andrea.molina@colegiocristobalcolon.edu.sv",
    "diego.rosales@colegiocristobalcolon.edu.sv",
    "anthony.menendez@colegiocristobalcolon.edu.sv",
    "erick.chavarria01@colegiocristobalcolon.edu.sv",
    "mateo.salgado@colegiocristobalcolon.edu.sv",
    "jorge.rodriguez@colegiocristobalcolon.edu.sv",
    "luciana.majano@colegiocristobalcolon.edu.sv",
    "andre.ramos@colegiocristobalcolon.edu.sv",
    "ariana.zaldivar@colegiocristobalcolon.edu.sv",
    "hector.catalan@colegiocristobalcolon.edu.sv",
    "rebeca.cordero01@colegiocristobalcolon.edu.sv",
    "alejandro.mendez@colegiocristobalcolon.edu.sv",
    "rodrigo.escobar@colegiocristobalcolon.edu.sv",
    "nelson.alvarado@colegiocristobalcolon.edu.sv",
    "julissa.rodriguez@colegiocristobalcolon.edu.sv",
    "marco.castillo@colegiocristobalcolon.edu.sv",
    "marlon.molina@colegiocristobalcolon.edu.sv",
    "sofia.lara@colegiocristobalcolon.edu.sv",
    "juan.castro@colegiocristobalcolon.edu.sv",
    "alessandra.rivera@colegiocristobalcolon.edu.sv",
    "michel.monterrosa01@colegiocristobalcolon.edu.sv",
    "matias.martinez@colegiocristobalcolon.edu.sv",
    "fiorella.perez@colegiocristobalcolon.edu.sv",
    "fernando.reyes@colegiocristobalcolon.edu.sv",
    "manuel.romero@colegiocristobalcolon.edu.sv",
    "ximena.pocasangre@colegiocristobalcolon.edu.sv",
    "ashlie.sigaran@colegiocristobalcolon.edu.sv",
    "ashly.diaz@colegiocristobalcolon.edu.sv",
    "alexia.duran@colegiocristobalcolon.edu.sv",
    "valentina.peña@colegiocristobalcolon.edu.sv",
    "maria.henriquez@colegiocristobalcolon.edu.sv",
    "mariana.abrego@colegiocristobalcolon.edu.sv",
    "gabriela.garcia@colegiocristobalcolon.edu.sv",
    "cesar.lopez@colegiocristobalcolon.edu.sv",
    "rafael.lopez@colegiocristobalcolon.edu.sv",
    "fernanda.reyes@colegiocristobalcolon.edu.sv",
    "eimy.rivas@colegiocristobalcolon.edu.sv",
    "roberto.arevalo@colegiocristobalcolon.edu.sv",
    "gabriel.polio@colegiocristobalcolon.edu.sv",
    "joanna.contreras@colegiocristobalcolon.edu.sv",
    "alan.reyes@colegiocristobalcolon.edu.sv",
    "edwin.rodríguez@colegiocristobalcolon.edu.sv",
    "fabio.velasquez@colegiocristobalcolon.edu.sv",
    "dianna.guevara@colegiocristobalcolon.edu.sv",
    "nicolas.erazo@colegiocristobalcolon.edu.sv",
    "andrea.melgar@colegiocristobalcolon.edu.sv",
    "hugo.duenas@colegiocristobalcolon.edu.sv",
    "fernanda.gonzalez@colegiocristobalcolon.edu.sv",
    "daniela.gutierrez@colegiocristobalcolon.edu.sv",
    "matias.flores@colegiocristobalcolon.edu.sv",
    "maria.villamariona@colegiocristobalcolon.edu.sv",
    "javier.hernandez@colegiocristobalcolon.edu.sv",
    "evans.hernandez@colegiocristobalcolon.edu.sv",
    "francisco.salazar@colegiocristobalcolon.edu.sv",
    "catalina.morales@colegiocristobalcolon.edu.sv",
    "gabriela.alas@colegiocristobalcolon.edu.sv",
    "danilo.rosales@colegiocristobalcolon.edu.sv",
    "hazel.martinez@colegiocristobalcolon.edu.sv",
    "carlos.bernal@colegiocristobalcolon.edu.sv",
    "lenin.rodriguez@colegiocristobalcolon.edu.sv",
    "alejandro.cobar@colegiocristobalcolon.edu.sv",
    "daniela.navarro@colegiocristobalcolon.edu.sv",
    "axel.alas@colegiocristobalcolon.edu.sv",
    "juan.lopez01@colegiocristobalcolon.edu.sv",
    "christopher.hernandez@colegiocristobalcolon.edu.sv",
    "adriana.castillo@colegiocristobalcolon.edu.sv",
    "stefany.santamaria@colegiocristobalcolon.edu.sv",
    "katheryn.duran@colegiocristobalcolon.edu.sv",
    "leo.melendez@colegiocristobalcolon.edu.sv",
    "adriel.chacon01@colegiocristobalcolon.edu.sv",
    "mariana.rosales@colegiocristobalcolon.edu.sv",
    "teresa.gonzalez@colegiocristobalcolon.edu.sv",
    "anneth.chavez@colegiocristobalcolon.edu.sv",
    "sebastian.rodriguez@colegiocristobalcolon.edu.sv",
    "marcelo.pineda@colegiocristobalcolon.edu.sv",
    "juan.alvarado@colegiocristobalcolon.edu.sv",
    "matilde.guevara@colegiocristobalcolon.edu.sv",
    "emilia.araujo@colegiocristobalcolon.edu.sv",
    "brenda.iraheta@colegiocristobalcolon.edu.sv",
    "camila.moran@colegiocristobalcolon.edu.sv",
    "leonardo.argueta@colegiocristobalcolon.edu.sv",
    "julio.argueta@colegiocristobalcolon.edu.sv",
    "fernanda.guevara@colegiocristobalcolon.edu.sv",
    "mariana.juarez@colegiocristobalcolon.edu.sv",
    "vivian.romero@colegiocristobalcolon.edu.sv",
    "carlos.jeronimo@colegiocristobalcolon.edu.sv",
    "gerardo.castillo @colegiocristobalcolon.edu.sv",
    "victor.hernandez@colegiocristobalcolon.edu.sv",
    "elenilson.vasquez@colegiocristobalcolon.edu.sv",
    "carlos.perez@colegiocristobalcolon.edu.sv",
    "lea.rivera@colegiocristobalcolon.edu.sv",
    "jose.molina@colegiocristobalcolon.edu.sv",
    "luis.hernandez@colegiocristobalcolon.edu.sv",
    "christopher.torres@colegiocristobalcolon.edu.sv",
    "aundrey.flores@colegiocristobalcolon.edu.sv",
    "matias.romero01@colegiocristobalcolon.edu.sv",
    "jesus.tellez@colegiocristobalcolon.edu.sv",
    "iveth.matamoros@colegiocristobalcolon.edu.sv",
    "steven.guardado@colegiocristobalcolon.edu.sv",
    "victor.marin@colegiocristobalcolon.edu.sv",
    "rafael.rivas@colegiocristobalcolon.edu.sv",
    "ashley.rivas@colegiocristobalcolon.edu.sv",
    "ximena.rosales@colegiocristobalcolon.edu.sv",
    "christopher.escobar@colegiocristobalcolon.edu.sv",
    "ashlee.cruz@colegiocristobalcolon.edu.sv",
    "daniela.salguero@colegiocristobalcolon.edu.sv",
    "amanda.villanueva@colegiocristobalcolon.edu.sv",
    "roxana.monterrosa@colegiocristobalcolon.edu.sv",
    "vasti.contreras@colegiocristobalcolon.edu.sv",
    "rodrigo.pleitez01@colegiocristobalcolon.edu.sv",
    "monica.molina@colegiocristobalcolon.edu.sv",
    "francisco.silva@colegiocristobalcolon.edu.sv",
    "lourdes.hernandez@colegiocristobalcolon.edu.sv",
    "fidel.morales@colegiocristobalcolon.edu.sv",
    "rudy.martinez@colegiocristobalcolon.edu.sv",
    "marco.alfaro@colegiocristobalcolon.edu.sv",
    "aisha.torres@colegiocristobalcolon.edu.sv",
    "ricardo.delgado@colegiocristobalcolon.edu.sv",
    "alessandra.mena@colegiocristobalcolon.edu.sv",
    "william.reyes@colegiocristobalcolon.edu.sv",
    "samuel.medrano@colegiocristobalcolon.edu.sv",
    "dennis.ramirez@colegiocristobalcolon.edu.sv",
    "allison.navas01@colegiocristobalcolon.edu.sv",
    "cristian.herrera@colegiocristobalcolon.edu.sv",
    "alvaro.ramirez01@colegiocristobalcolon.edu.sv",
    "mia.villalta@colegiocristobalcolon.edu.sv",
    "sylvia.recinos@colegiocristobalcolon.edu.sv",
    "steffany.leiva@colegiocristobalcolon.edu.sv",
    "fabiola.contreras@colegiocristobalcolon.edu.sv",
    "helen.alda@colegiocristobalcolon.edu.sv",
    "fernanda.martinez@colegiocristobalcolon.edu.sv",
    "gabriel.serrano@colegiocristobalcolon.edu.sv",
    "rodrigo.mejia@colegiocristobalcolon.edu.sv",
    "francisco.garcia@colegiocristobalcolon.edu.sv",
    "marcelo.duran@colegiocristobalcolon.edu.sv",
    "johanna.gonzalez@colegiocristobalcolon.edu.sv",
    "maria.rivera@colegiocristobalcolon.edu.sv",
    "gerson.velasquez@colegiocristobalcolon.edu.sv",
    "ariana.alberto@colegiocristobalcolon.edu.sv",
    "anderson.rosales@colegiocristobalcolon.edu.sv",
    "rodrigo.quinteros@colegiocristobalcolon.edu.sv",
    "ana.urrutia@colegiocristobalcolon.edu.sv",
    "carlos.reyes@colegiocristobalcolon.edu.sv",
    "maria.carillo@colegiocristobalcolon.edu.sv",
    "luis.granadeno01@colegiocristobalcolon.edu.sv",
    "santiago.serrano@colegiocristobalcolon.edu.sv"
];

// let users = [];

let index = 286;
// for (let index = 0; index < 555; index++) {
    let school_id = 'f5c3c010-b381-4835-adbe-890ddb60995f'; // Colegio Cristobal Colon
    let inventory_group_id = '4a43badc-a0ec-49d0-b735-c803143f03cd';
    let group_id = getGroupId(data[0]['data'][index][3], data[0]['data'][index][4]);
    
    let first_name = data[0].data[index][0];
    let last_name = data[0].data[index][1];
    let full_name = first_name + ' ' + last_name;
    let pic_path = 'https://files.roboxmaker.com/uploads/avatar.png';
    let email = data[0].data[index][2];
    let password = data[0].data[index][5].toString();
    let role = 'student';
    let user_id = data[0].data[index][8];
    let license = data[0].data[index][7];
    let username = data[0].data[index][5].toString();

    // let grado = data[0].data[index][3];
    // let section = data[0].data[index][4];

    // console.log(index, data[0].data[index][2]);

    // if (!user_exists.includes(email)) {
        // const data = {
        //     first_name,
        //     last_name,
        //     email,
        //     grado,
        //     section, 
        //     username,
        //     password,
        //     license,
        // };

        // users.push(data);

        // console.log(index, data[0].data[index][2]);

        
        let cmd = `npm run aker-gql-op -- new_user_create --user_id "${user_id}" --license "${license}" --full_name "${full_name}" --pic_path "${pic_path}" --first_name "${first_name}" --last_name "${last_name}" --email "${email}" --password "${password}" --role "${role}" --school_id "${school_id}" --group_id "${group_id}" --inventory_group_id "${inventory_group_id}"  --username "${username}"`;
        console.log(cmd);
        
        // console.log(index, typeof username, typeof password);
        setTimeout(() => {
            // console.log("Delayed for 1 second.");
        }, "3000")
    
        exec(cmd, (error, stdout, stderr) => {
            if (error) {
                console.log(`error: ${error.message}`);
                return;
            }
            if (stderr) {
                console.log(`stderr: ${stderr}`);
                return;
            }
            console.log(`stdout: ${stdout}`);
        });
    // }
// }

// const ws = XLSX.utils.json_to_sheet(users);

// const wb = XLSX.utils.book_new();
// XLSX.utils.book_append_sheet(wb, ws, 'Usuarios');

// XLSX.writeFile(wb, 'userExists.xlsx');