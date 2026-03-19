const {
    exec
} = require("child_process");
const fs = require('fs');

var XLSX = require('xlsx');

const rows = [];

// We read the JSON file
fs.readFile('quiz_responses.json', 'utf8', (err, data) => {
    if (err) {
        console.error('Error reading file:', err);
        return;
    }

    // We parse the JSON file
    const resp = JSON.parse(data);

    // console.log("Response:", resp.data);

    const userList = resp.data.quizzes[0];

    // ========== Usuarios que respondieron ==========
    const usersSheet = userList.quiz_responses.map(response => {
        const profile = response.user?.user_profile;
        return {
            Nombre: profile?.full_name || "Desconocido",
            Email: profile?.email || "Sin email",
        };
    });

    // ========== Resumen de respuestas por pregunta ==========
    const questionStats = {}; // { question_id: { question, opciones: { option_id: { texto, total } } } }

    for (const response of resp.data.quiz_responses) {
        for (const answer of response.user_answers) {
            const qid = answer.question_id;
            const questionText = answer.question?.question;

            if (!questionStats[qid]) {
            questionStats[qid] = {
                question: questionText,
                options: {}
            };

            // Inicializar las opciones
            for (const opt of answer.question?.answer_options || []) {
                questionStats[qid].options[opt.id] = {
                texto: opt.option,
                total: 0
                };
            }
            }

            // Incrementar el contador dependiendo del tipo de respuesta
            if (answer.answer_type === "SINGLE_CHOICE") {
                const selected = answer.single_choice_option_id;
                if (questionStats[qid].options[selected]) {
                    questionStats[qid].options[selected].total++;
                }
            } else if (answer.answer_type === "MULTIPLE_CHOICE") {
                for (const userChoice of answer.user_multiple_choices || []) {
                    const selected = userChoice.option_id;
                    if (questionStats[qid].options[selected]) {
                    questionStats[qid].options[selected].total++;
                    }
                }
            }
        }
    }

    // Convertir estadísticas a una hoja
    const questionsSheet = [];
    for (const [qid, qData] of Object.entries(questionStats)) {
        questionsSheet.push({ Pregunta: qData.question });
        for (const [optId, opt] of Object.entries(qData.options)) {
            questionsSheet.push({
            Opción: opt.texto,
            Total: opt.total
            });
        }
        questionsSheet.push({}); // Fila vacía para separar preguntas
    }

    // ========== Crear el libro Excel ==========
    const workbook = XLSX.utils.book_new();

    const usersWS = XLSX.utils.json_to_sheet(usersSheet);
    XLSX.utils.book_append_sheet(workbook, usersWS, "Usuarios");

    const questionsWS = XLSX.utils.json_to_sheet(questionsSheet);
    XLSX.utils.book_append_sheet(workbook, questionsWS, "Respuestas");

    XLSX.writeFile(workbook, "reporte_quizz.xlsx");
});
