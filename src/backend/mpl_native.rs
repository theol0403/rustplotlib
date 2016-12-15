use std::io;
use backend::Backend;
use cpython::{GILGuard, Python, PyModule, PyTuple, PyDict};


pub struct MatplotlibNative {
  gil: GILGuard,
  plt: PyModule,
}

impl MatplotlibNative {
  pub fn new() -> MatplotlibNative {
    let gil = Python::acquire_gil();
    let plt;
    {
      let py = gil.python();
      plt = PyModule::import(py, "matplotlib.pyplot").unwrap();
    }
    MatplotlibNative {
      gil: gil,
      plt: plt,
    }
  }

  pub fn python<'a>(&'a self) -> Python<'a> {
    self.gil.python()
  }

  pub fn exec<S: AsRef<str>>(&mut self, script: S) -> io::Result<&mut Self> {
    self.python().run(script.as_ref(), None, None).unwrap();
    Ok(self)
  }
}

impl Backend for MatplotlibNative {
  /// call `plt.figure()` to create a instance of `matplotlib.figure.Figure`.
  fn figure(&mut self) -> io::Result<&mut Self> {
    self.plt.call(self.python(), "figure", PyTuple::empty(self.python()), None).unwrap();
    Ok(self)
  }

  fn savefig(&mut self, filename: &str) -> io::Result<&mut Self> {
    self.plt.call(self.python(), "savefig", (filename,), None).unwrap();
    Ok(self)
  }

  fn show(&mut self) -> io::Result<&mut Self> {
    self.plt.call(self.python(), "show", PyTuple::empty(self.python()), None).unwrap();
    Ok(self)
  }

  fn subplot(&mut self, i: u32, j: u32, k: u32) -> io::Result<&mut Self> {
    self.plt.call(self.python(), "subplot", (i, j, k), None).unwrap();
    Ok(self)
  }

  fn grid(&mut self, grid: bool) -> io::Result<&mut Self> {
    self.plt.call(self.python(), "grid", (grid,), None).unwrap();
    Ok(self)
  }

  fn legend(&mut self, loc: &str) -> io::Result<&mut Self> {
    let kwargs = PyDict::new(self.python());
    kwargs.set_item(self.python(), "loc", loc).unwrap();
    self.plt
      .call(self.python(),
            "legend",
            PyTuple::empty(self.python()),
            Some(&kwargs))
      .unwrap();
    Ok(self)
  }

  fn xlim(&mut self, xlim: &(f64, f64)) -> io::Result<&mut Self> {
    self.plt.call(self.python(), "xlim", xlim, None).unwrap();
    Ok(self)
  }

  fn ylim(&mut self, ylim: &(f64, f64)) -> io::Result<&mut Self> {
    self.plt.call(self.python(), "ylim", ylim, None).unwrap();
    Ok(self)
  }

  fn scatter(&mut self,
             xdata: &[f64],
             ydata: &[f64],
             label: &Option<String>,
             color: &Option<String>,
             marker: &Option<String>)
             -> io::Result<&mut Self> {
    let kwargs = PyDict::new(self.python());
    kwargs.set_item(self.python(), "label", label).unwrap();
    kwargs.set_item(self.python(), "color", color).unwrap();
    kwargs.set_item(self.python(), "marker", marker).unwrap();
    self.plt.call(self.python(), "scatter", (xdata, ydata), Some(&kwargs)).unwrap();
    Ok(self)
  }

  fn plot(&mut self,
          xdata: &[f64],
          ydata: &[f64],
          label: &Option<String>,
          color: &Option<String>,
          marker: &Option<String>,
          linestyle: &Option<String>,
          linewidth: &Option<f64>)
          -> io::Result<&mut Self> {
    let kwargs = PyDict::new(self.python());
    kwargs.set_item(self.python(), "label", label).unwrap();
    kwargs.set_item(self.python(), "color", color).unwrap();
    kwargs.set_item(self.python(), "marker", marker).unwrap();
    kwargs.set_item(self.python(), "ls", linestyle).unwrap();
    kwargs.set_item(self.python(), "lw", linewidth).unwrap();
    self.plt.call(self.python(), "plot", (xdata, ydata), Some(&kwargs)).unwrap();
    Ok(self)
  }

  fn set_style(&mut self, stylename: &str) -> io::Result<&mut Self> {
    use cpython::FromPyObject;
    let style = self.plt
      .get(self.python(), "style")
      .and_then(|ref style| PyModule::extract(self.python(), style))
      .unwrap();
    style.call(self.python(), "use", (stylename,), None).unwrap();
    Ok(self)
  }
}
