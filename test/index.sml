@include "test/second.sml"

html {
  head {
    title { "My Personal Website" }
  }
  body {
    h1 { "This is a heading" }
    p { "Here is some text" }
    div {
      p { "First paragraph in the div" }
      p { "Second paragraph in the div" }
      a [
        href = "https://www.ky5.com"
        style = ""
      ] { "have a nice day!" }
      @new-component {
        p { "inside component" }
      }
    }
  }
}
