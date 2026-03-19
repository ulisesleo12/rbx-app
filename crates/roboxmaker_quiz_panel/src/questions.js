// export function render_questions(node, questions_stats) {
//     console.log('SSSSSSSSSSSSSSSSS', questions_stats);

//     const result = new Map();

//     questions_stats.forEach((question, i) => {
//         const counts = new Map(question.option_counts);

//         // We get all the options in this question
//         const options = Array.from(counts.keys());

//         // We assure that all options are on the result map
//         options.forEach(option => {
//             if (!result.has(option)) {
//                 result.set(option, []);
//             }
//         });

//         // For each option already known (accumulated in previous questions too)
//         result.forEach((data, option) => {
//             // If the option is in this question, we add the real value
//             data.push(counts.get(option) || 0); // If it is not there, 0 is added
//         });
//     });

//     const series = Array.from(result.entries()).map(([name, data]) => ({
//         name,
//         data
//     }));

//     const questions = questions_stats.map((_, index) => `Q${index + 1}`);

//     var options = {
//         series: series,
//         chart: {
//             type: 'bar',
//             height: 350,
//             stacked: true,
//         },
//         plotOptions: {
//             bar: {
//                 horizontal: false,
//                 dataLabels: {
//                     total: {
//                         enabled: true,
//                         offsetX: 0,
//                         style: {
//                             fontSize: '13px',
//                             fontWeight: 900
//                         }
//                     }
//                 }
//             },
//         },
//         stroke: {
//             width: 1,
//             colors: ['#fff']
//         },
//         title: {
//             text: 'Aciertos o errores por pregunta',
//             align: 'center',
//             style: {
//                 fontSize:  '16px',
//                 fontWeight:  'bold',
//                 color:  '#022754'
//             },
//         },
//         xaxis: {
//             categories: questions,
//             labels: {
//                 formatter: function (val) {
//                     return val
//                 }
//             }
//         },
//         yaxis: {
//             title: {
//                 text: undefined
//             },
//         },
//         grid: {
//             show: true,
//             xaxis: {
//                 lines: {
//                     show: true
//                 }
//             },
//             yaxis: {
//                 lines: {
//                     show: true
//                 }
//             }
//         },
//         tooltip: {
//             y: {
//                 formatter: function (val) {
//                     return val
//                 } 
//             }
//         },
//         fill: {
//             opacity: 1
//         },
//         legend: {
//             position: 'top',
//             horizontalAlign: 'left',
//             offsetX: 40
//         }
//     };

//     var chart = new ApexCharts(node, options);
//     chart.render();
// }
export function render_questions(node, questions_stats) {
    console.log('SSSSSSSSSSSSSSSSS', questions_stats);

    const allOpinionBased = questions_stats.every(q => q.is_opinion_based);

    if (allOpinionBased) {
        render_opinion_chart(node, questions_stats);
    } else if (!allOpinionBased) {
        render_objective_chart(node, questions_stats);
    } else {
        render_questions_combined(node, questions_stats);
    }
}


export function render_opinion_chart(node, questions_stats) {
    const result = new Map();

    questions_stats.forEach((question, i) => {
        const counts = new Map(question.option_counts);

        // We get all the options in this question
        const options = Array.from(counts.keys());

        // We assure that all options are on the result map
        options.forEach(option => {
            if (!result.has(option)) {
                result.set(option, []);
            }
        });

        // For each option already known (accumulated in previous questions too)
        result.forEach((data, option) => {
            // If the option is in this question, we add the real value
            data.push(counts.get(option) || 0); // If it is not there, 0 is added
        });
    });

    const series = Array.from(result.entries()).map(([name, data]) => ({
        name,
        data
    }));

    const questions = questions_stats.map((_, index) => `Q${index + 1}`);

    var options = {
        series: series,
        chart: {
            type: 'bar',
            height: 350,
            stacked: true,
        },
        plotOptions: {
            bar: {
                horizontal: false,
                dataLabels: {
                    total: {
                        enabled: true,
                        offsetX: 0,
                        style: {
                            fontSize: '13px',
                            fontWeight: 900
                        }
                    }
                }
            },
        },
        stroke: {
            width: 1,
            colors: ['#fff']
        },
        title: {
            text: 'Aciertos o errores por pregunta',
            align: 'center',
            style: {
                fontSize:  '16px',
                fontWeight:  'bold',
                color:  '#022754'
            },
        },
        xaxis: {
            categories: questions,
            labels: {
                formatter: function (val) {
                    return val
                }
            }
        },
        yaxis: {
            title: {
                text: undefined
            },
        },
        grid: {
            show: true,
            xaxis: {
                lines: {
                    show: true
                }
            },
            yaxis: {
                lines: {
                    show: true
                }
            }
        },
        tooltip: {
            y: {
                formatter: function (val) {
                    return val
                } 
            }
        },
        fill: {
            opacity: 1
        },
        legend: {
            position: 'top',
            horizontalAlign: 'left',
            offsetX: 40
        }
    };

    new ApexCharts(node, options).render();
}


function render_objective_chart(node, questions_stats) {
    const categories = questions_stats.map((_, i) => `Q${i + 1}`);

    const series = [
        {
            name: "Correctas",
            data: questions_stats.map(q => q.correct_count || 0),
        },
        {
            name: "Incorrectas",
            data: questions_stats.map(q => q.incorrect_count || 0),
        },
    ];

    var options = {
        series: series,
        chart: {
            type: 'bar',
            height: 350,
            stacked: true,
        },
        plotOptions: {
            bar: {
                horizontal: false,
                dataLabels: {
                    total: {
                        enabled: true,
                        offsetX: 0,
                        style: {
                            fontSize: '13px',
                            fontWeight: 900
                        }
                    }
                }
            },
        },
        stroke: {
            width: 1,
            colors: ['#fff']
        },
        title: {
            text: 'Aciertos o errores por pregunta',
            align: 'center',
            style: {
                fontSize:  '16px',
                fontWeight:  'bold',
                color:  '#022754'
            },
        },
        xaxis: {
            categories: categories,
            labels: {
                formatter: function (val) {
                    return val
                }
            }
        },
        yaxis: {
            title: {
                text: undefined
            },
        },
        grid: {
            show: true,
            xaxis: {
                lines: {
                    show: true
                }
            },
            yaxis: {
                lines: {
                    show: true
                }
            }
        },
        tooltip: {
            y: {
                formatter: function (val) {
                    return val
                } 
            }
        },
        fill: {
            opacity: 1
        },
        legend: {
            position: 'top',
            horizontalAlign: 'left',
            offsetX: 40
        }
    };

    new ApexCharts(node, options).render();
}


export function render_questions_combined(node, questions_stats) {
    const x_labels = questions_stats.map((_, index) => `Q${index + 1}`);

    const series_map = new Map();

    questions_stats.forEach((q, index) => {
        if (q.is_opinion_based) {
            // opinion-based: usa option_counts
            q.option_counts.forEach(([option, count]) => {
                if (!series_map.has(option)) {
                    series_map.set(option, Array(questions_stats.length).fill(0));
                }
                series_map.get(option)[index] = count;
            });
        } else {
            // objective: usa correct_count y incorrect_count
            const correct_label = "Correctas";
            const incorrect_label = "Incorrectas";

            if (!series_map.has(correct_label)) {
                series_map.set(correct_label, Array(questions_stats.length).fill(0));
            }
            if (!series_map.has(incorrect_label)) {
                series_map.set(incorrect_label, Array(questions_stats.length).fill(0));
            }

            series_map.get(correct_label)[index] = q.correct_count || 0;
            series_map.get(incorrect_label)[index] = q.incorrect_count || 0;
        }
    });

    const series = Array.from(series_map.entries()).map(([name, data]) => ({
        name,
        data
    }));

    const options = {
        series,
        chart: {
            type: 'bar',
            height: 450,
            stacked: true,
        },
        title: {
            text: 'Resumen de respuestas por pregunta',
            align: 'center',
            style: {
                fontSize: '16px',
                fontWeight: 'bold'
            }
        },
        xaxis: {
            categories: x_labels,
        },
        yaxis: {
            title: {
                text: 'Cantidad de respuestas'
            }
        },
        legend: {
            position: 'top',
            horizontalAlign: 'left'
        },
        tooltip: {
            y: {
                formatter: val => val
            }
        },
        fill: {
            opacity: 1
        }
    };

    new ApexCharts(node, options).render();
}


export function render_n_users(node, scores) {
    console.log('VVVVVVVVVVVVVVV', scores);

    const maxX = Math.max(...scores.map(([x, _]) => x));
    const data = Array.from({ length: maxX + 1 }, (_, i) => {
        const found = scores.find(([x]) => x === i);
        return found ? found[1] : 0;
    });

    const categories = Array.from({ length: maxX + 1 }, (_, i) => i);

    var options = {
        series: [
            {
                data: data
            }
        ],
        chart: {
            type: 'bar',
            height: 350
        },
        plotOptions: {
            bar: {
                horizontal: false,
            }
        },
        dataLabels: {
            enabled: true
        },
        title: {
            text: 'Distribución de la puntuación',
            align: 'center',
            style: {
                fontSize:  '16px',
                fontWeight:  'bold',
                color:  '#022754'
            },
        },
        xaxis: {
            categories: categories,
            title: {
                text: 'Puntuación',
            },
        },
        grid: {
            show: true,
            xaxis: {
                lines: {
                    show: true
                }
            },
            yaxis: {
                lines: {
                    show: true
                }
            }
        },
        yaxis: {
            title: {
                text: 'Nª de alumnos',
            },
            forceNiceScale: false,
            tickAmount: 1,
            min: 0,
        }
    };

    var chart = new ApexCharts(node, options);
    chart.render();
}