// import { createApp } from "vue";
let daySelected = null;
export function activity_vcalendar(node_vcal, date, id, dateActivitySelected) {
  node_vcal.innerHTML = `<v-date-picker v-model="selectedDate" color="gray" @dayclick="onDayClick" :input-debounce="500" :masks="masks" class="d-flex flex-wrap">
      <template v-slot="{ inputValue, inputEvents }">
        <div class="text-purple-gray is-size-20 px-2 vc-calendar-act">
          <i class="far fa-calendar"></i>
        </div>
        <input class="bg-white border px-2 py-1 input input-style-universal input-style-vcalendar" :value="inputValue" v-on="inputEvents" />
      </template>
    </v-date-picker>`

  // console.log(node_vcal);
  // console.log(date);
  // console.log(dateActivitySelected);
  dateActivitySelected(date);
  new Vue({
    // el: "#AppVcal",
    el: `#${id}`,
    data: {
      selectedDate: Date(),
      // selectedDate: date,
      masks: {
        input: "DD-MM-YYYY"
      }
    },
    methods: {
      onDayClick(day) {
        if (daySelected != day.id) {
          dateActivitySelected(day.id);
          daySelected = day.id;
        }
      },
    },
  })
}