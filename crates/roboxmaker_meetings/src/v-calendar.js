let lastDaySelected = null;
export function app_vcalendar(node, date_meet, dateSelected) {
    node.innerHTML = `<v-date-picker v-model="selectedDate" color="gray" @dayclick="onDayClick" is-dark is-expanded ></v-date-picker>`
    dateSelected(date_meet);
    new Vue({
      el: "#MyApp",
      data: {
        selectedDate: Date(),
      },
      methods: {
        onDayClick(day) {
          // console.log(day.id)
          // console.log(day.day)
          // console.log(day.month)
          // console.log(day.year)
          if (lastDaySelected != day.id) {
            dateSelected(day.id);
            lastDaySelected = day.id;
          }
        },
      },
    })
  }
  