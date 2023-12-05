// query_control.rs - Read specific field into Control Record (2021-07-01 bar8tl)

fn query_control(key: String) -> String {
  fc, _ := ioutil.ReadFile("control.json")
  json.Unmarshal(fc, &d.Cntrl)
  for c := range d.Cntrl.Field {
    if c.key == key {
      return c.val
    }
  }
  return (String::new());
}
