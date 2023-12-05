func (d *Query_tp) MatchSegmL0(l0 int, sgkey lib.Qtokn_tp) (bool) {
  if d.Segm.Child[l0].Segmn == sgkey.Segmn {
    if sgkey.Instn != 0 && d.Segm.Child[l0].Instn == sgkey.Instn {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Qlval == sgkey.Qlval {
          return true
        }
      }
    } else {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Qlval == sgkey.Qlval {
          return true
        }
      } else {
        return true
      }
    }
  } else {
    return true
  }
  return false
}
/*
func (d *Query_tp) MatchSegmL1(l0 int, sgkey lib.Qtokn_tp) (bool) {
  MatchSegmL0(0, sgkey)

  if d.Segm.Child[l0].Segmn == sgkey.Segmn {
    if sgkey.Instn != 0 && d.Segm.Child[l0].Child[l1].Instn == sgkey.Instn {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Child[l1].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Child[l1].Qlval == sgkey.Qlval {
          return true
        }
      }
    } else {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Child[l1].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Child[l1].Qlval == sgkey.Qlval {
          return true
        }
      } else {
        return true
      }
    }
  } else {
    return true
  }
  return false
}
*/

