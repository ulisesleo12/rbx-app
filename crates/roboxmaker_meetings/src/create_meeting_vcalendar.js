export function activity_vcalendar(node_create, today, dateActivitySelected) {
    node_create.innerHTML = `<v-date-picker v-model="selectedDate" color="purple" @dayclick="onDayClick" :input-debounce="500" :masks="masks" class="d-flex flex-wrap" is-dark>
      <template v-slot="{ inputValue, inputEvents }">
        <div class="text-purple-gray is-size-20 px-2 vc-calendar-act">
          <i class="far fa-calendar"></i>
        </div>
        <input class="bg-white border px-2 py-1 input input-style-universal input-style-vcalendar" :value="inputValue" v-on="inputEvents" style="width: 280px;" />
      </template>
    </v-date-picker>`

  dateActivitySelected(today);
  new Vue({
    el: "#VC-Meet",
    data: {
      selectedDate: Date(),
      masks: {
        input: "DD-MM-YYYY"
      }
    },
    methods: {
      onDayClick(day) {
        dateActivitySelected(day.id);
      },
    },
  })
}