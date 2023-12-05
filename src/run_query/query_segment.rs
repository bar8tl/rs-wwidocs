func (d *Query_tp) QuerySegment(sgkey lib.Qtokn_tp, key string) (string) {
  fs, _ := ioutil.ReadFile("segment.json")
  json.Unmarshal(fs, &d.Segm)
  if d.MatchSegmL0(0, sgkey) {
    for _, f := range d.Segm.Child[0].Field {
      if f.Key == key {
        return f.Val
      }
    }
  }
  return ""
}
