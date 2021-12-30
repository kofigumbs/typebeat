

pub struct insertSIG0 {
	iVec4: [i32;2],
	iRec27: [i32;2],
}

impl insertSIG0 {
	
	fn get_num_inputsinsertSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsinsertSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initinsertSIG0(&mut self, sample_rate: i32) {
		let mut l29: i32 = 0;
		loop {
			self.iVec4[l29 as usize] = 0;
			l29 = (l29 + 1);
			if (l29 < 2) { continue; } else { break; }
		}
		let mut l30: i32 = 0;
		loop {
			self.iRec27[l30 as usize] = 0;
			l30 = (l30 + 1);
			if (l30 < 2) { continue; } else { break; }
		}
	}
	
	fn fillinsertSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec4[0] = 1;
			self.iRec27[0] = ((self.iVec4[1] + self.iRec27[1]) % 65536);
			table[i1 as usize] = F32::sin((9.58738019e-05 * (self.iRec27[0] as F32)));
			self.iVec4[1] = self.iVec4[0];
			self.iRec27[1] = self.iRec27[0];
		}
	}

}


pub fn newinsertSIG0() -> insertSIG0 { 
	insertSIG0 {
		iVec4: [0;2],
		iRec27: [0;2],
	}
}
static mut ftbl0insertSIG0: [F32;65536] = [0.0;65536];
fn insert_faustpower2_f(value: F32) -> F32 {
	return (value * value);
}
pub struct insert {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	iRec1: [i32;2],
	fConst2: F32,
	fEntry0: F32,
	iVec2: [i32;2],
	fEntry1: F32,
	fRec0: [F32;2],
	fEntry2: F32,
	fRec2: [F32;2],
	fEntry3: F32,
	fRec4: [F32;2],
	fEntry4: F32,
	fRec5: [F32;2],
	fRec6: [F32;2],
	fEntry5: F32,
	fRec7: [F32;2],
	fEntry6: F32,
	fRec8: [F32;2],
	fEntry7: F32,
	fRec9: [F32;2],
	iRec10: [i32;2],
	fRec11: [F32;2],
	fRec3: [F32;2],
	fEntry8: F32,
	fRec13: [F32;2],
	fEntry9: F32,
	fRec14: [F32;2],
	fRec12: [F32;2],
	fEntry10: F32,
	fRec17: [F32;2],
	fRec16: [F32;2],
	fEntry11: F32,
	fRec19: [F32;2],
	fEntry12: F32,
	fButton1: F32,
	fEntry13: F32,
	fRec21: [F32;2],
	fRec20: [F32;2],
	IOTA: i32,
	fVec3: [F32;131072],
	fRec22: [F32;2],
	fEntry14: F32,
	fEntry15: F32,
	fRec23: [F32;2],
	fEntry16: F32,
	fRec24: [F32;2],
	fRec25: [F32;2],
	fRec26: [F32;2],
	fConst3: F32,
	fEntry17: F32,
	fRec29: [F32;2],
	fRec28: [F32;2],
	fConst4: F32,
	fConst5: F32,
	fConst6: F32,
	fRec31: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fConst7: F32,
	fRec30: [F32;2],
	fRec32: [F32;2],
	fRec34: [F32;2],
	iRec35: [i32;2],
	fEntry18: F32,
	fRec36: [F32;2],
	fEntry19: F32,
	fRec37: [F32;2],
	fRec38: [F32;2],
	fRec39: [F32;2],
	fEntry20: F32,
	fRec41: [F32;2],
	fRec40: [F32;2],
	fRec43: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec42: [F32;2],
	fRec44: [F32;2],
	fConst8: F32,
	fRec46: [F32;2],
	fEntry21: F32,
	fRec47: [F32;2],
	fEntry22: F32,
	fRec48: [F32;2],
	fRec49: [F32;2],
	fRec50: [F32;2],
	fEntry23: F32,
	fRec52: [F32;2],
	fRec51: [F32;2],
	fRec54: [F32;2],
	fVec9: [F32;2],
	fVec10: [F32;4096],
	fRec53: [F32;2],
	fRec55: [F32;2],
	fRec57: [F32;2],
	fConst9: F32,
	fEntry24: F32,
	fRec58: [F32;2],
	fRec18: [F32;3],
	fConst10: F32,
	fRec15: [F32;3],
	fEntry25: F32,
	fRec60: [F32;2],
	fRec59: [F32;3],
	fVec11: [F32;131072],
	fRec63: [F32;3],
	fRec62: [F32;3],
	fRec61: [F32;3],
	fEntry26: F32,
	fRec64: [F32;2],
	fEntry27: F32,
	fRec67: [F32;2],
	fConst11: F32,
	fRec66: [F32;2],
	fRec65: [F32;2],
	fRec69: [F32;2],
	fRec68: [F32;2],
	fEntry28: F32,
	fRec70: [F32;2],
	fEntry29: F32,
	fRec71: [F32;2],
}

impl FaustDsp for insert {
	type T = F32;
		
	fn new() -> insert { 
		insert {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			iRec1: [0;2],
			fConst2: 0.0,
			fEntry0: 0.0,
			iVec2: [0;2],
			fEntry1: 0.0,
			fRec0: [0.0;2],
			fEntry2: 0.0,
			fRec2: [0.0;2],
			fEntry3: 0.0,
			fRec4: [0.0;2],
			fEntry4: 0.0,
			fRec5: [0.0;2],
			fRec6: [0.0;2],
			fEntry5: 0.0,
			fRec7: [0.0;2],
			fEntry6: 0.0,
			fRec8: [0.0;2],
			fEntry7: 0.0,
			fRec9: [0.0;2],
			iRec10: [0;2],
			fRec11: [0.0;2],
			fRec3: [0.0;2],
			fEntry8: 0.0,
			fRec13: [0.0;2],
			fEntry9: 0.0,
			fRec14: [0.0;2],
			fRec12: [0.0;2],
			fEntry10: 0.0,
			fRec17: [0.0;2],
			fRec16: [0.0;2],
			fEntry11: 0.0,
			fRec19: [0.0;2],
			fEntry12: 0.0,
			fButton1: 0.0,
			fEntry13: 0.0,
			fRec21: [0.0;2],
			fRec20: [0.0;2],
			IOTA: 0,
			fVec3: [0.0;131072],
			fRec22: [0.0;2],
			fEntry14: 0.0,
			fEntry15: 0.0,
			fRec23: [0.0;2],
			fEntry16: 0.0,
			fRec24: [0.0;2],
			fRec25: [0.0;2],
			fRec26: [0.0;2],
			fConst3: 0.0,
			fEntry17: 0.0,
			fRec29: [0.0;2],
			fRec28: [0.0;2],
			fConst4: 0.0,
			fConst5: 0.0,
			fConst6: 0.0,
			fRec31: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fConst7: 0.0,
			fRec30: [0.0;2],
			fRec32: [0.0;2],
			fRec34: [0.0;2],
			iRec35: [0;2],
			fEntry18: 0.0,
			fRec36: [0.0;2],
			fEntry19: 0.0,
			fRec37: [0.0;2],
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fEntry20: 0.0,
			fRec41: [0.0;2],
			fRec40: [0.0;2],
			fRec43: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec42: [0.0;2],
			fRec44: [0.0;2],
			fConst8: 0.0,
			fRec46: [0.0;2],
			fEntry21: 0.0,
			fRec47: [0.0;2],
			fEntry22: 0.0,
			fRec48: [0.0;2],
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fEntry23: 0.0,
			fRec52: [0.0;2],
			fRec51: [0.0;2],
			fRec54: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fRec53: [0.0;2],
			fRec55: [0.0;2],
			fRec57: [0.0;2],
			fConst9: 0.0,
			fEntry24: 0.0,
			fRec58: [0.0;2],
			fRec18: [0.0;3],
			fConst10: 0.0,
			fRec15: [0.0;3],
			fEntry25: 0.0,
			fRec60: [0.0;2],
			fRec59: [0.0;3],
			fVec11: [0.0;131072],
			fRec63: [0.0;3],
			fRec62: [0.0;3],
			fRec61: [0.0;3],
			fEntry26: 0.0,
			fRec64: [0.0;2],
			fEntry27: 0.0,
			fRec67: [0.0;2],
			fConst11: 0.0,
			fRec66: [0.0;2],
			fRec65: [0.0;2],
			fRec69: [0.0;2],
			fRec68: [0.0;2],
			fEntry28: 0.0,
			fRec70: [0.0;2],
			fEntry29: 0.0,
			fRec71: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/name", "Faust Analyzer Library");
		m.declare("analyzers.lib/version", "0.1");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("envelopes.lib/adsr:author", "Yann Orlarey and Andrey Bundin");
		m.declare("envelopes.lib/ar:author", "Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", "GRAME");
		m.declare("envelopes.lib/copyright", "GRAME");
		m.declare("envelopes.lib/license", "LGPL with exception");
		m.declare("envelopes.lib/name", "Faust Envelope Library");
		m.declare("envelopes.lib/version", "0.1");
		m.declare("filename", "insert.dsp");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/pole:author", "Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/version", "0.3");
		m.declare("interpolators.lib/name", "Faust Interpolator Library");
		m.declare("interpolators.lib/version", "0.3");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("misceffects.lib/name", "Misc Effects Library");
		m.declare("misceffects.lib/version", "2.0");
		m.declare("name", "insert");
		m.declare("noises.lib/name", "Faust Noise Generator Library");
		m.declare("noises.lib/version", "0.0");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/version", "0.1");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("routes.lib/name", "Faust Signal Routing Library");
		m.declare("routes.lib/version", "0.2");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("spats.lib/name", "Faust Spatialization Library");
		m.declare("spats.lib/version", "0.0");
		m.declare("webaudio.lib/author", "GRAME");
		m.declare("webaudio.lib/copyright", "GRAME");
		m.declare("webaudio.lib/license", "LGPL with exception");
		m.declare("webaudio.lib/name", "WebAudio Filters Library");
		m.declare("webaudio.lib/version", "0.1");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 4;
	}
	fn get_num_outputs(&self) -> i32 {
		return 8;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: insertSIG0 = newinsertSIG0();
		sig0.instance_initinsertSIG0(sample_rate);
		sig0.fillinsertSIG0(65536, unsafe { &mut ftbl0insertSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fEntry0 = 0.0;
		self.fEntry1 = 0.0;
		self.fEntry2 = 0.0;
		self.fEntry3 = 50.0;
		self.fEntry4 = 0.0;
		self.fEntry5 = 0.0;
		self.fEntry6 = 50.0;
		self.fEntry7 = 0.0;
		self.fEntry8 = 0.0;
		self.fEntry9 = 0.0;
		self.fEntry10 = 0.0;
		self.fEntry11 = 25.0;
		self.fEntry12 = 0.0;
		self.fButton1 = 0.0;
		self.fEntry13 = 0.0;
		self.fEntry14 = 0.0;
		self.fEntry15 = 0.0;
		self.fEntry16 = 0.0;
		self.fEntry17 = 0.0;
		self.fEntry18 = 0.0;
		self.fEntry19 = 0.0;
		self.fEntry20 = 0.0;
		self.fEntry21 = 0.0;
		self.fEntry22 = 0.0;
		self.fEntry23 = 0.0;
		self.fEntry24 = 0.0;
		self.fEntry25 = 0.0;
		self.fEntry26 = 50.0;
		self.fEntry27 = 0.0;
		self.fEntry28 = 0.0;
		self.fEntry29 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[l1 as usize] = 0;
		}
		for l2 in 0..2 {
			self.iRec1[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.iVec2[l3 as usize] = 0;
		}
		for l4 in 0..2 {
			self.fRec0[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec2[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec4[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec5[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec6[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec7[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec8[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec9[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.iRec10[l12 as usize] = 0;
		}
		for l13 in 0..2 {
			self.fRec11[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec3[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec13[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec14[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec12[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec17[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec16[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec19[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec21[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec20[l22 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l23 in 0..131072 {
			self.fVec3[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec22[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec23[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec24[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec25[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec26[l28 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec29[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec28[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec31[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fVec5[l34 as usize] = 0.0;
		}
		for l35 in 0..4096 {
			self.fVec6[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec30[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec32[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec34[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.iRec35[l39 as usize] = 0;
		}
		for l40 in 0..2 {
			self.fRec36[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec37[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec38[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec39[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec41[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec40[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec43[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fVec7[l47 as usize] = 0.0;
		}
		for l48 in 0..4096 {
			self.fVec8[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec42[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec44[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec46[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec47[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec48[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec49[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec50[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec52[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec51[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec54[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fVec9[l59 as usize] = 0.0;
		}
		for l60 in 0..4096 {
			self.fVec10[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec53[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec55[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec57[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec58[l64 as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec18[l65 as usize] = 0.0;
		}
		for l66 in 0..3 {
			self.fRec15[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec60[l67 as usize] = 0.0;
		}
		for l68 in 0..3 {
			self.fRec59[l68 as usize] = 0.0;
		}
		for l69 in 0..131072 {
			self.fVec11[l69 as usize] = 0.0;
		}
		for l70 in 0..3 {
			self.fRec63[l70 as usize] = 0.0;
		}
		for l71 in 0..3 {
			self.fRec62[l71 as usize] = 0.0;
		}
		for l72 in 0..3 {
			self.fRec61[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec64[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec67[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec66[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec65[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec69[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec68[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec70[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec71[l80 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)));
		self.fConst1 = (1.0 - (44.0999985 / self.fConst0));
		self.fConst2 = (0.0199999996 * self.fConst0);
		self.fConst3 = (440.0 / self.fConst0);
		self.fConst4 = (1760.0 / self.fConst0);
		self.fConst5 = (0.25 * self.fConst0);
		self.fConst6 = (1.0 / self.fConst0);
		self.fConst7 = (0.5 * self.fConst0);
		self.fConst8 = (0.125 * self.fConst0);
		self.fConst9 = (880.0 / self.fConst0);
		self.fConst10 = (2.0 / self.fConst0);
		self.fConst11 = F32::exp((0.0 - self.fConst10));
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		insert::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("insert");
		ui_interface.add_num_entry("attack", ParamIndex(0), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("cutoff", ParamIndex(1), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("decay", ParamIndex(2), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("duckBy", ParamIndex(3), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("echo", ParamIndex(4), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_button("gate", ParamIndex(5));
		ui_interface.add_num_entry("highFreq", ParamIndex(6), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("highRes", ParamIndex(7), 0.0, -50.0, 0.0, 10.0);
		ui_interface.add_num_entry("holdSample", ParamIndex(8), 0.0, 0.0, 1.0, 0.0);
		ui_interface.add_num_entry("lowFreq", ParamIndex(9), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("lowRes", ParamIndex(10), 0.0, -50.0, 0.0, 10.0);
		ui_interface.add_num_entry("main", ParamIndex(11), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("midFreq", ParamIndex(12), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("midRes", ParamIndex(13), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_button("note", ParamIndex(14));
		ui_interface.add_num_entry("pan", ParamIndex(15), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("release", ParamIndex(16), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("reverb", ParamIndex(17), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("sampleDetune", ParamIndex(18), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("sampleLevel", ParamIndex(19), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("sampleType", ParamIndex(20), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("sustain", ParamIndex(21), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Detune", ParamIndex(22), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth1Level", ParamIndex(23), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Type", ParamIndex(24), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth2Detune", ParamIndex(25), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth2Level", ParamIndex(26), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth2Type", ParamIndex(27), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth3Detune", ParamIndex(28), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth3Level", ParamIndex(29), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth3Type", ParamIndex(30), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("toDuck", ParamIndex(31), 0.0, 0.0, 50.0, 10.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			5 => Some(self.fButton0),
			14 => Some(self.fButton1),
			16 => Some(self.fEntry0),
			31 => Some(self.fEntry1),
			9 => Some(self.fEntry10),
			19 => Some(self.fEntry11),
			20 => Some(self.fEntry12),
			18 => Some(self.fEntry13),
			8 => Some(self.fEntry14),
			26 => Some(self.fEntry15),
			27 => Some(self.fEntry16),
			25 => Some(self.fEntry17),
			23 => Some(self.fEntry18),
			24 => Some(self.fEntry19),
			15 => Some(self.fEntry2),
			22 => Some(self.fEntry20),
			29 => Some(self.fEntry21),
			30 => Some(self.fEntry22),
			28 => Some(self.fEntry23),
			10 => Some(self.fEntry24),
			7 => Some(self.fEntry25),
			11 => Some(self.fEntry26),
			3 => Some(self.fEntry27),
			17 => Some(self.fEntry28),
			4 => Some(self.fEntry29),
			6 => Some(self.fEntry3),
			1 => Some(self.fEntry4),
			0 => Some(self.fEntry5),
			21 => Some(self.fEntry6),
			2 => Some(self.fEntry7),
			12 => Some(self.fEntry8),
			13 => Some(self.fEntry9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			5 => { self.fButton0 = value }
			14 => { self.fButton1 = value }
			16 => { self.fEntry0 = value }
			31 => { self.fEntry1 = value }
			9 => { self.fEntry10 = value }
			19 => { self.fEntry11 = value }
			20 => { self.fEntry12 = value }
			18 => { self.fEntry13 = value }
			8 => { self.fEntry14 = value }
			26 => { self.fEntry15 = value }
			27 => { self.fEntry16 = value }
			25 => { self.fEntry17 = value }
			23 => { self.fEntry18 = value }
			24 => { self.fEntry19 = value }
			15 => { self.fEntry2 = value }
			22 => { self.fEntry20 = value }
			29 => { self.fEntry21 = value }
			30 => { self.fEntry22 = value }
			28 => { self.fEntry23 = value }
			10 => { self.fEntry24 = value }
			7 => { self.fEntry25 = value }
			11 => { self.fEntry26 = value }
			3 => { self.fEntry27 = value }
			17 => { self.fEntry28 = value }
			4 => { self.fEntry29 = value }
			6 => { self.fEntry3 = value }
			1 => { self.fEntry4 = value }
			0 => { self.fEntry5 = value }
			21 => { self.fEntry6 = value }
			2 => { self.fEntry7 = value }
			12 => { self.fEntry8 = value }
			13 => { self.fEntry9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2, inputs3) = if let [inputs0, inputs1, inputs2, inputs3, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			let inputs3 = inputs3[..count as usize].iter();
			(inputs0, inputs1, inputs2, inputs3)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			let outputs3 = outputs3[..count as usize].iter_mut();
			let outputs4 = outputs4[..count as usize].iter_mut();
			let outputs5 = outputs5[..count as usize].iter_mut();
			let outputs6 = outputs6[..count as usize].iter_mut();
			let outputs7 = outputs7[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = (self.fButton0 as F32);
		let mut fSlow1: F32 = (self.fEntry0 as F32);
		let mut fSlow2: F32 = (1.0 / F32::max(1.0, (self.fConst2 * fSlow1)));
		let mut fSlow3: F32 = (self.fEntry1 as F32);
		let mut fSlow4: F32 = (self.fEntry2 as F32);
		let mut fSlow5: F32 = (self.fEntry3 as F32);
		let mut fSlow6: F32 = (self.fEntry4 as F32);
		let mut fSlow7: F32 = (self.fEntry5 as F32);
		let mut fSlow8: F32 = (self.fEntry6 as F32);
		let mut fSlow9: F32 = (self.fEntry7 as F32);
		let mut iSlow10: i32 = ((fSlow0 == 0.0) as i32);
		let mut fSlow11: F32 = (self.fEntry8 as F32);
		let mut fSlow12: F32 = (self.fEntry9 as F32);
		let mut fSlow13: F32 = (self.fEntry10 as F32);
		let mut fSlow14: F32 = (self.fEntry11 as F32);
		let mut fSlow15: F32 = (self.fEntry12 as F32);
		let mut fSlow16: F32 = (((((fSlow15 == ((1 | (fSlow15 as i32)) as F32)) as i32) == 2) as i32) as F32);
		let mut fSlow17: F32 = (self.fButton1 as F32);
		let mut fSlow18: F32 = (self.fEntry13 as F32);
		let mut fSlow19: F32 = (0.0833333358 * fSlow16);
		let mut iSlow20: i32 = ((self.fEntry14 as F32) as i32);
		let mut fSlow21: F32 = (self.fEntry15 as F32);
		let mut fSlow22: F32 = (self.fEntry16 as F32);
		let mut fSlow23: F32 = (((fSlow22 >= 3.0) as i32) as F32);
		let mut fSlow24: F32 = (((fSlow22 >= 2.0) as i32) as F32);
		let mut fSlow25: F32 = (((fSlow22 >= 1.0) as i32) as F32);
		let mut fSlow26: F32 = (self.fEntry17 as F32);
		let mut fSlow27: F32 = (((fSlow22 >= 4.0) as i32) as F32);
		let mut fSlow28: F32 = (self.fEntry18 as F32);
		let mut fSlow29: F32 = (self.fEntry19 as F32);
		let mut fSlow30: F32 = (((fSlow29 >= 3.0) as i32) as F32);
		let mut fSlow31: F32 = (((fSlow29 >= 2.0) as i32) as F32);
		let mut fSlow32: F32 = (((fSlow29 >= 1.0) as i32) as F32);
		let mut fSlow33: F32 = (self.fEntry20 as F32);
		let mut fSlow34: F32 = (((fSlow29 >= 4.0) as i32) as F32);
		let mut fSlow35: F32 = (self.fEntry21 as F32);
		let mut fSlow36: F32 = (self.fEntry22 as F32);
		let mut fSlow37: F32 = (((fSlow36 >= 3.0) as i32) as F32);
		let mut fSlow38: F32 = (((fSlow36 >= 2.0) as i32) as F32);
		let mut fSlow39: F32 = (((fSlow36 >= 1.0) as i32) as F32);
		let mut fSlow40: F32 = (self.fEntry23 as F32);
		let mut fSlow41: F32 = (((fSlow36 >= 4.0) as i32) as F32);
		let mut fSlow42: F32 = (self.fEntry24 as F32);
		let mut fSlow43: F32 = (self.fEntry25 as F32);
		let mut fSlow44: F32 = (self.fEntry26 as F32);
		let mut fSlow45: F32 = (self.fEntry27 as F32);
		let mut fSlow46: F32 = (self.fEntry28 as F32);
		let mut fSlow47: F32 = (self.fEntry29 as F32);
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7);
		for (((((((((((input0, input1), input2), input3), output0), output1), output2), output3), output4), output5), output6), output7) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			self.iVec1[0] = 1;
			self.iRec1[0] = (((fSlow0 > self.fVec0[1]) as i32) + (((fSlow0 <= self.fVec0[1]) as i32) * (self.iRec1[1] + ((self.iRec1[1] > 0) as i32))));
			let mut fTemp0: F32 = (self.iRec1[0] as F32);
			let mut fTemp1: F32 = F32::max(0.0, F32::min(fTemp0, ((fSlow2 * (1.0 - fTemp0)) + 1.0)));
			let mut iTemp2: i32 = (((fTemp1 > 0.0) as i32) - ((fTemp1 < 0.0) as i32));
			self.iVec2[0] = iTemp2;
			let mut fTemp3: F32 = ((((iTemp2 == self.iVec2[1]) as i32) | ((iTemp2 == 0) as i32)) as F32);
			let mut fTemp4: F32 = (1.0 - (self.fConst1 * fTemp3));
			self.fRec0[0] = ((self.fConst1 * (fTemp3 * self.fRec0[1])) + (fSlow3 * fTemp4));
			self.fRec2[0] = ((self.fConst1 * (self.fRec2[1] * fTemp3)) + (fSlow4 * fTemp4));
			let mut iTemp5: i32 = ((self.fRec2[0] > 25.0) as i32);
			let mut fTemp6: F32 = (self.fRec3[1] + 1.0);
			self.fRec4[0] = ((self.fConst1 * (fTemp3 * self.fRec4[1])) + (fSlow5 * fTemp4));
			self.fRec5[0] = ((self.fConst1 * (fTemp3 * self.fRec5[1])) + (fSlow6 * fTemp4));
			self.fRec6[0] = (fSlow0 + (self.fRec6[1] * (((self.fVec0[1] >= fSlow0) as i32) as F32)));
			self.fRec7[0] = ((self.fConst1 * (fTemp3 * self.fRec7[1])) + (fSlow7 * fTemp4));
			let mut fTemp7: F32 = F32::max(1.0, (self.fConst2 * self.fRec7[0]));
			self.fRec8[0] = ((self.fConst1 * (fTemp3 * self.fRec8[1])) + (fSlow8 * fTemp4));
			let mut fTemp8: F32 = (0.0199999996 * self.fRec8[0]);
			self.fRec9[0] = ((self.fConst1 * (fTemp3 * self.fRec9[1])) + (fSlow9 * fTemp4));
			self.iRec10[0] = (iSlow10 * (self.iRec10[1] + 1));
			self.fRec11[0] = ((self.fConst1 * (fTemp3 * self.fRec11[1])) + (fSlow1 * fTemp4));
			let mut fTemp9: F32 = F32::max(0.0, (F32::min((self.fRec6[0] / fTemp7), F32::max(((((1.0 - fTemp8) * (fTemp7 - self.fRec6[0])) / F32::max(1.0, (self.fConst2 * self.fRec9[0]))) + 1.0), fTemp8)) * (1.0 - ((self.iRec10[0] as F32) / F32::max(1.0, (self.fConst2 * self.fRec11[0]))))));
			let mut fTemp10: F32 = (self.fRec5[0] * fTemp9);
			let mut fTemp11: F32 = ((((self.fRec4[0] - fTemp10) == 50.0) as i32) as F32);
			let mut fTemp12: F32 = (self.fRec3[1] + -1.0);
			self.fRec3[0] = if (((fTemp6 < fTemp11) as i32) as i32 != 0) { fTemp6 } else { if (((fTemp12 > fTemp11) as i32) as i32 != 0) { fTemp12 } else { fTemp11 } };
			let mut fTemp13: F32 = (self.fRec12[1] + 1.0);
			self.fRec13[0] = ((self.fConst1 * (fTemp3 * self.fRec13[1])) + (fSlow11 * fTemp4));
			self.fRec14[0] = ((self.fConst1 * (fTemp3 * self.fRec14[1])) + (fSlow12 * fTemp4));
			let mut fTemp14: F32 = ((((self.fRec13[0] == 0.0) as i32) & ((self.fRec14[0] == 0.0) as i32)) as F32);
			let mut fTemp15: F32 = (self.fRec12[1] + -1.0);
			self.fRec12[0] = if (((fTemp13 < fTemp14) as i32) as i32 != 0) { fTemp13 } else { if (((fTemp15 > fTemp14) as i32) as i32 != 0) { fTemp15 } else { fTemp14 } };
			let mut fTemp16: F32 = (1.0 - self.fRec12[0]);
			let mut fTemp17: F32 = (self.fRec16[1] + 1.0);
			self.fRec17[0] = ((self.fConst1 * (fTemp3 * self.fRec17[1])) + (fSlow13 * fTemp4));
			let mut fTemp18: F32 = (((self.fRec17[0] == 0.0) as i32) as F32);
			let mut fTemp19: F32 = (self.fRec16[1] + -1.0);
			self.fRec16[0] = if (((fTemp17 < fTemp18) as i32) as i32 != 0) { fTemp17 } else { if (((fTemp19 > fTemp18) as i32) as i32 != 0) { fTemp19 } else { fTemp18 } };
			let mut fTemp20: F32 = (1.0 - self.fRec16[0]);
			self.fRec19[0] = ((self.fConst1 * (fTemp3 * self.fRec19[1])) + (fSlow14 * fTemp4));
			let mut fTemp21: F32 = (*input2 as F32);
			let mut fTemp22: F32 = (self.fRec20[1] + 1.0);
			self.fRec21[0] = ((self.fConst1 * (fTemp3 * self.fRec21[1])) + (fSlow18 * fTemp4));
			let mut fTemp23: F32 = (fSlow17 + ((0.100000001 * self.fRec21[0]) + -69.0));
			let mut fTemp24: F32 = ((((fSlow16 * fTemp23) == 0.0) as i32) as F32);
			let mut fTemp25: F32 = (self.fRec20[1] + -1.0);
			self.fRec20[0] = if (((fTemp22 < fTemp24) as i32) as i32 != 0) { fTemp22 } else { if (((fTemp25 > fTemp24) as i32) as i32 != 0) { fTemp25 } else { fTemp24 } };
			let mut fTemp26: F32 = (1.0 - self.fRec20[0]);
			let mut fTemp27: F32 = (fTemp21 * fTemp26);
			self.fVec3[(self.IOTA & 131071) as usize] = fTemp27;
			self.fRec22[0] = libm::fmodf((self.fRec22[1] + (1001.0 - F32::powf(2.0, (fSlow19 * fTemp23)))), 1000.0);
			let mut iTemp28: i32 = (self.fRec22[0] as i32);
			let mut iTemp29: i32 = std::cmp::min(65537, (std::cmp::max(0, (iTemp28 as i32)) as i32));
			let mut fTemp30: F32 = F32::floor(self.fRec22[0]);
			let mut fTemp31: F32 = (fTemp30 + (1.0 - self.fRec22[0]));
			let mut fTemp32: F32 = (self.fRec22[0] - fTemp30);
			let mut iTemp33: i32 = std::cmp::min(65537, (std::cmp::max(0, ((iTemp28 + 1) as i32)) as i32));
			let mut fTemp34: F32 = F32::min((0.100000001 * self.fRec22[0]), 1.0);
			let mut fTemp35: F32 = (self.fRec22[0] + 1000.0);
			let mut iTemp36: i32 = (fTemp35 as i32);
			let mut iTemp37: i32 = std::cmp::min(65537, (std::cmp::max(0, (iTemp36 as i32)) as i32));
			let mut fTemp38: F32 = F32::floor(fTemp35);
			let mut fTemp39: F32 = (fTemp38 + (-999.0 - self.fRec22[0]));
			let mut iTemp40: i32 = std::cmp::min(65537, (std::cmp::max(0, ((iTemp36 + 1) as i32)) as i32));
			let mut fTemp41: F32 = (self.fRec22[0] + (1000.0 - fTemp38));
			let mut fTemp42: F32 = (1.0 - fTemp34);
			let mut fTemp43: F32 = if (iSlow20 as i32 != 0) { fTemp9 } else { 1.0 };
			self.fRec23[0] = ((self.fConst1 * (fTemp3 * self.fRec23[1])) + (fSlow21 * fTemp4));
			let mut fTemp44: F32 = (self.fRec24[1] + 1.0);
			let mut fTemp45: F32 = (self.fRec24[1] + -1.0);
			self.fRec24[0] = if (((fTemp44 < fSlow23) as i32) as i32 != 0) { fTemp44 } else { if (((fTemp45 > fSlow23) as i32) as i32 != 0) { fTemp45 } else { fSlow23 } };
			let mut fTemp46: F32 = (self.fRec25[1] + 1.0);
			let mut fTemp47: F32 = (self.fRec25[1] + -1.0);
			self.fRec25[0] = if (((fTemp46 < fSlow24) as i32) as i32 != 0) { fTemp46 } else { if (((fTemp47 > fSlow24) as i32) as i32 != 0) { fTemp47 } else { fSlow24 } };
			let mut fTemp48: F32 = (self.fRec26[1] + 1.0);
			let mut fTemp49: F32 = (self.fRec26[1] + -1.0);
			self.fRec26[0] = if (((fTemp48 < fSlow25) as i32) as i32 != 0) { fTemp48 } else { if (((fTemp49 > fSlow25) as i32) as i32 != 0) { fTemp49 } else { fSlow25 } };
			self.fRec29[0] = ((self.fConst1 * (fTemp3 * self.fRec29[1])) + (fSlow26 * fTemp4));
			let mut fTemp50: F32 = F32::powf(2.0, (0.0833333358 * (fSlow17 + ((0.00999999978 * self.fRec29[0]) + -69.0))));
			let mut fTemp51: F32 = (self.fRec28[1] + (self.fConst3 * fTemp50));
			self.fRec28[0] = (fTemp51 - F32::floor(fTemp51));
			let mut fTemp52: F32 = (self.iVec1[1] as F32);
			let mut fTemp53: F32 = (440.0 * fTemp50);
			let mut fTemp54: F32 = F32::max(fTemp53, 23.4489498);
			let mut fTemp55: F32 = F32::max(20.0, F32::abs(fTemp54));
			let mut fTemp56: F32 = (self.fRec31[1] + (self.fConst6 * fTemp55));
			self.fRec31[0] = (fTemp56 - F32::floor(fTemp56));
			let mut fTemp57: F32 = insert_faustpower2_f(((2.0 * self.fRec31[0]) + -1.0));
			self.fVec5[0] = fTemp57;
			let mut fTemp58: F32 = ((fTemp52 * (fTemp57 - self.fVec5[1])) / fTemp55);
			self.fVec6[(self.IOTA & 4095) as usize] = fTemp58;
			let mut fTemp59: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst7 / fTemp54)));
			let mut iTemp60: i32 = (fTemp59 as i32);
			let mut fTemp61: F32 = F32::floor(fTemp59);
			let mut fTemp62: F32 = (self.fConst5 * (((self.fVec6[((self.IOTA - iTemp60) & 4095) as usize] * (fTemp61 + (1.0 - fTemp59))) - fTemp58) + ((fTemp59 - fTemp61) * self.fVec6[((self.IOTA - (iTemp60 + 1)) & 4095) as usize])));
			self.fRec30[0] = ((0.999000013 * self.fRec30[1]) - fTemp62);
			let mut fTemp63: F32 = F32::max(1.1920929e-07, F32::abs(fTemp53));
			let mut fTemp64: F32 = (self.fConst6 * fTemp63);
			let mut fTemp65: F32 = (self.fRec32[1] + fTemp64);
			let mut fTemp66: F32 = (fTemp65 + -1.0);
			let mut iTemp67: i32 = ((fTemp66 < 0.0) as i32);
			self.fRec32[0] = if (iTemp67 as i32 != 0) { fTemp65 } else { fTemp66 };
			let mut fRec33: F32 = if (iTemp67 as i32 != 0) { fTemp65 } else { (fTemp64 + (self.fRec32[1] + (fTemp66 * (1.0 - (self.fConst0 / fTemp63))))) };
			let mut fTemp68: F32 = (self.fRec34[1] + 1.0);
			let mut fTemp69: F32 = (self.fRec34[1] + -1.0);
			self.fRec34[0] = if (((fTemp68 < fSlow27) as i32) as i32 != 0) { fTemp68 } else { if (((fTemp69 > fSlow27) as i32) as i32 != 0) { fTemp69 } else { fSlow27 } };
			self.iRec35[0] = ((1103515245 * self.iRec35[1]) + 12345);
			let mut fTemp70: F32 = (self.iRec35[0] as F32);
			self.fRec36[0] = ((self.fConst1 * (fTemp3 * self.fRec36[1])) + (fSlow28 * fTemp4));
			let mut fTemp71: F32 = (self.fRec37[1] + 1.0);
			let mut fTemp72: F32 = (self.fRec37[1] + -1.0);
			self.fRec37[0] = if (((fTemp71 < fSlow30) as i32) as i32 != 0) { fTemp71 } else { if (((fTemp72 > fSlow30) as i32) as i32 != 0) { fTemp72 } else { fSlow30 } };
			let mut fTemp73: F32 = (self.fRec38[1] + 1.0);
			let mut fTemp74: F32 = (self.fRec38[1] + -1.0);
			self.fRec38[0] = if (((fTemp73 < fSlow31) as i32) as i32 != 0) { fTemp73 } else { if (((fTemp74 > fSlow31) as i32) as i32 != 0) { fTemp74 } else { fSlow31 } };
			let mut fTemp75: F32 = (self.fRec39[1] + 1.0);
			let mut fTemp76: F32 = (self.fRec39[1] + -1.0);
			self.fRec39[0] = if (((fTemp75 < fSlow32) as i32) as i32 != 0) { fTemp75 } else { if (((fTemp76 > fSlow32) as i32) as i32 != 0) { fTemp76 } else { fSlow32 } };
			self.fRec41[0] = ((self.fConst1 * (fTemp3 * self.fRec41[1])) + (fSlow33 * fTemp4));
			let mut fTemp77: F32 = F32::powf(2.0, (0.0833333358 * (fSlow17 + ((0.00999999978 * self.fRec41[0]) + -69.0))));
			let mut fTemp78: F32 = (self.fRec40[1] + (self.fConst3 * fTemp77));
			self.fRec40[0] = (fTemp78 - F32::floor(fTemp78));
			let mut fTemp79: F32 = (440.0 * fTemp77);
			let mut fTemp80: F32 = F32::max(fTemp79, 23.4489498);
			let mut fTemp81: F32 = F32::max(20.0, F32::abs(fTemp80));
			let mut fTemp82: F32 = (self.fRec43[1] + (self.fConst6 * fTemp81));
			self.fRec43[0] = (fTemp82 - F32::floor(fTemp82));
			let mut fTemp83: F32 = insert_faustpower2_f(((2.0 * self.fRec43[0]) + -1.0));
			self.fVec7[0] = fTemp83;
			let mut fTemp84: F32 = ((fTemp52 * (fTemp83 - self.fVec7[1])) / fTemp81);
			self.fVec8[(self.IOTA & 4095) as usize] = fTemp84;
			let mut fTemp85: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst7 / fTemp80)));
			let mut iTemp86: i32 = (fTemp85 as i32);
			let mut fTemp87: F32 = F32::floor(fTemp85);
			let mut fTemp88: F32 = ((fTemp84 - (self.fVec8[((self.IOTA - iTemp86) & 4095) as usize] * (fTemp87 + (1.0 - fTemp85)))) - ((fTemp85 - fTemp87) * self.fVec8[((self.IOTA - (iTemp86 + 1)) & 4095) as usize]));
			self.fRec42[0] = ((0.999000013 * self.fRec42[1]) + (self.fConst5 * fTemp88));
			let mut fTemp89: F32 = F32::max(1.1920929e-07, F32::abs(fTemp79));
			let mut fTemp90: F32 = (self.fRec44[1] + (self.fConst6 * fTemp89));
			let mut fTemp91: F32 = (fTemp90 + -1.0);
			let mut iTemp92: i32 = ((fTemp91 < 0.0) as i32);
			self.fRec44[0] = if (iTemp92 as i32 != 0) { fTemp90 } else { fTemp91 };
			let mut fRec45: F32 = if (iTemp92 as i32 != 0) { fTemp90 } else { (fTemp90 + (fTemp91 * (1.0 - (self.fConst0 / fTemp89)))) };
			let mut fTemp93: F32 = (self.fRec46[1] + 1.0);
			let mut fTemp94: F32 = (self.fRec46[1] + -1.0);
			self.fRec46[0] = if (((fTemp93 < fSlow34) as i32) as i32 != 0) { fTemp93 } else { if (((fTemp94 > fSlow34) as i32) as i32 != 0) { fTemp94 } else { fSlow34 } };
			self.fRec47[0] = ((self.fConst1 * (fTemp3 * self.fRec47[1])) + (fSlow35 * fTemp4));
			let mut fTemp95: F32 = (self.fRec48[1] + 1.0);
			let mut fTemp96: F32 = (self.fRec48[1] + -1.0);
			self.fRec48[0] = if (((fTemp95 < fSlow37) as i32) as i32 != 0) { fTemp95 } else { if (((fTemp96 > fSlow37) as i32) as i32 != 0) { fTemp96 } else { fSlow37 } };
			let mut fTemp97: F32 = (self.fRec49[1] + 1.0);
			let mut fTemp98: F32 = (self.fRec49[1] + -1.0);
			self.fRec49[0] = if (((fTemp97 < fSlow38) as i32) as i32 != 0) { fTemp97 } else { if (((fTemp98 > fSlow38) as i32) as i32 != 0) { fTemp98 } else { fSlow38 } };
			let mut fTemp99: F32 = (self.fRec50[1] + 1.0);
			let mut fTemp100: F32 = (self.fRec50[1] + -1.0);
			self.fRec50[0] = if (((fTemp99 < fSlow39) as i32) as i32 != 0) { fTemp99 } else { if (((fTemp100 > fSlow39) as i32) as i32 != 0) { fTemp100 } else { fSlow39 } };
			self.fRec52[0] = ((self.fConst1 * (fTemp3 * self.fRec52[1])) + (fSlow40 * fTemp4));
			let mut fTemp101: F32 = F32::powf(2.0, (0.0833333358 * (fSlow17 + ((0.00999999978 * self.fRec52[0]) + -69.0))));
			let mut fTemp102: F32 = (self.fRec51[1] + (self.fConst3 * fTemp101));
			self.fRec51[0] = (fTemp102 - F32::floor(fTemp102));
			let mut fTemp103: F32 = (440.0 * fTemp101);
			let mut fTemp104: F32 = F32::max(fTemp103, 23.4489498);
			let mut fTemp105: F32 = F32::max(20.0, F32::abs(fTemp104));
			let mut fTemp106: F32 = (self.fRec54[1] + (self.fConst6 * fTemp105));
			self.fRec54[0] = (fTemp106 - F32::floor(fTemp106));
			let mut fTemp107: F32 = insert_faustpower2_f(((2.0 * self.fRec54[0]) + -1.0));
			self.fVec9[0] = fTemp107;
			let mut fTemp108: F32 = ((fTemp52 * (fTemp107 - self.fVec9[1])) / fTemp105);
			self.fVec10[(self.IOTA & 4095) as usize] = fTemp108;
			let mut fTemp109: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst7 / fTemp104)));
			let mut iTemp110: i32 = (fTemp109 as i32);
			let mut fTemp111: F32 = F32::floor(fTemp109);
			let mut fTemp112: F32 = ((fTemp108 - (self.fVec10[((self.IOTA - iTemp110) & 4095) as usize] * (fTemp111 + (1.0 - fTemp109)))) - ((fTemp109 - fTemp111) * self.fVec10[((self.IOTA - (iTemp110 + 1)) & 4095) as usize]));
			self.fRec53[0] = ((0.999000013 * self.fRec53[1]) + (self.fConst5 * fTemp112));
			let mut fTemp113: F32 = F32::max(1.1920929e-07, F32::abs(fTemp103));
			let mut fTemp114: F32 = (self.fRec55[1] + (self.fConst6 * fTemp113));
			let mut fTemp115: F32 = (fTemp114 + -1.0);
			let mut iTemp116: i32 = ((fTemp115 < 0.0) as i32);
			self.fRec55[0] = if (iTemp116 as i32 != 0) { fTemp114 } else { fTemp115 };
			let mut fRec56: F32 = if (iTemp116 as i32 != 0) { fTemp114 } else { (fTemp114 + (fTemp115 * (1.0 - (self.fConst0 / fTemp113)))) };
			let mut fTemp117: F32 = (self.fRec57[1] + 1.0);
			let mut fTemp118: F32 = (self.fRec57[1] + -1.0);
			self.fRec57[0] = if (((fTemp117 < fSlow41) as i32) as i32 != 0) { fTemp117 } else { if (((fTemp118 > fSlow41) as i32) as i32 != 0) { fTemp118 } else { fSlow41 } };
			let mut fTemp119: F32 = (0.00666666683 * (fTemp9 * (((self.fRec23[0] * (((1.0 - self.fRec24[0]) * (((1.0 - self.fRec25[0]) * (((1.0 - self.fRec26[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec28[0]) as i32) as usize] }) + (self.fConst4 * ((self.fRec26[0] * self.fRec30[0]) * fTemp50)))) + (0.5 * (self.fRec25[0] * ((2.0 * fRec33) + -1.0))))) + (self.fRec24[0] * ((0.5 * ((1.0 - self.fRec34[0]) * (0.0 - fTemp62))) + (1.16415322e-10 * (self.fRec34[0] * fTemp70)))))) + (self.fRec36[0] * (((1.0 - self.fRec37[0]) * (((1.0 - self.fRec38[0]) * (((1.0 - self.fRec39[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec40[0]) as i32) as usize] }) + (self.fConst4 * ((self.fRec39[0] * self.fRec42[0]) * fTemp77)))) + (0.5 * (self.fRec38[0] * ((2.0 * fRec45) + -1.0))))) + (self.fRec37[0] * ((self.fConst8 * ((1.0 - self.fRec46[0]) * fTemp88)) + (1.16415322e-10 * (self.fRec46[0] * fTemp70))))))) + (self.fRec47[0] * (((1.0 - self.fRec48[0]) * (((1.0 - self.fRec49[0]) * (((1.0 - self.fRec50[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec51[0]) as i32) as usize] }) + (self.fConst4 * ((self.fRec50[0] * self.fRec53[0]) * fTemp101)))) + (0.5 * (self.fRec49[0] * ((2.0 * fRec56) + -1.0))))) + (self.fRec48[0] * ((self.fConst8 * ((1.0 - self.fRec57[0]) * fTemp112)) + (1.16415322e-10 * (self.fRec57[0] * fTemp70)))))))));
			let mut fTemp120: F32 = ((0.0399999991 * ((self.fRec19[0] * ((fTemp21 * self.fRec20[0]) + (fTemp26 * ((((self.fVec3[((self.IOTA - iTemp29) & 131071) as usize] * fTemp31) + (fTemp32 * self.fVec3[((self.IOTA - iTemp33) & 131071) as usize])) * fTemp34) + (((self.fVec3[((self.IOTA - iTemp37) & 131071) as usize] * fTemp39) + (self.fVec3[((self.IOTA - iTemp40) & 131071) as usize] * fTemp41)) * fTemp42))))) * fTemp43)) + fTemp119);
			let mut fTemp121: F32 = (3.14159274 * F32::max(0.00999999978, F32::min((self.fConst9 * F32::powf(2.0, (0.0833333358 * ((2.0 * self.fRec17[0]) + -79.0)))), 0.99000001)));
			let mut fTemp122: F32 = F32::cos(fTemp121);
			let mut fTemp123: F32 = (0.0 - (2.0 * fTemp122));
			self.fRec58[0] = ((self.fConst1 * (fTemp3 * self.fRec58[1])) + (fSlow42 * fTemp4));
			let mut fTemp124: F32 = (0.5 * (F32::sin(fTemp121) * F32::powf(10.0, (0.0 - (0.0500000007 * self.fRec58[0])))));
			let mut fTemp125: F32 = (1.0 - fTemp124);
			let mut fTemp126: F32 = (fTemp124 + 1.0);
			self.fRec18[0] = ((fTemp20 * fTemp120) - (((self.fRec18[1] * fTemp123) + (self.fRec18[2] * fTemp125)) / fTemp126));
			let mut fTemp127: F32 = (-1.0 - fTemp122);
			let mut fTemp128: F32 = (0.0 - (0.5 * fTemp127));
			let mut fTemp129: F32 = (((fTemp20 * (((self.fRec18[0] * fTemp128) + (self.fRec18[1] * fTemp127)) + (self.fRec18[2] * fTemp128))) / fTemp126) + (self.fRec16[0] * fTemp120));
			let mut fTemp130: F32 = (3.14159274 * F32::max(0.00999999978, F32::min((self.fConst10 * ((0.5 * self.fRec13[0]) + 60.0)), 0.99000001)));
			let mut fTemp131: F32 = (0.0 - (2.0 * F32::cos(fTemp130)));
			let mut fTemp132: F32 = (self.fRec15[1] * fTemp131);
			let mut fTemp133: F32 = F32::sin(fTemp130);
			let mut fTemp134: F32 = F32::powf(10.0, (0.0125000002 * self.fRec14[0]));
			let mut fTemp135: F32 = (0.125 * (fTemp133 / fTemp134));
			let mut fTemp136: F32 = (1.0 - fTemp135);
			let mut fTemp137: F32 = (fTemp135 + 1.0);
			self.fRec15[0] = ((fTemp16 * fTemp129) - ((fTemp132 + (self.fRec15[2] * fTemp136)) / fTemp137));
			let mut fTemp138: F32 = (0.125 * (fTemp133 * fTemp134));
			let mut fTemp139: F32 = (fTemp138 + 1.0);
			let mut fTemp140: F32 = (1.0 - fTemp138);
			let mut fTemp141: F32 = (((fTemp16 * ((fTemp132 + (self.fRec15[0] * fTemp139)) + (self.fRec15[2] * fTemp140))) / fTemp137) + (self.fRec12[0] * fTemp129));
			let mut fTemp142: F32 = (self.fRec3[0] * fTemp141);
			let mut fTemp143: F32 = (1.0 - self.fRec3[0]);
			let mut fTemp144: F32 = (3.14159274 * F32::max(0.00999999978, F32::min((self.fConst9 * F32::powf(2.0, (0.0833333358 * (self.fRec4[0] + (-9.0 - fTemp10))))), 0.99000001)));
			let mut fTemp145: F32 = F32::cos(fTemp144);
			let mut fTemp146: F32 = (fTemp143 * (1.0 - fTemp145));
			let mut fTemp147: F32 = (0.0 - (2.0 * fTemp145));
			self.fRec60[0] = ((self.fConst1 * (fTemp3 * self.fRec60[1])) + (fSlow43 * fTemp4));
			let mut fTemp148: F32 = (0.5 * (F32::sin(fTemp144) * F32::powf(10.0, (0.0 - (0.0500000007 * self.fRec60[0])))));
			let mut fTemp149: F32 = (1.0 - fTemp148);
			let mut fTemp150: F32 = (fTemp148 + 1.0);
			self.fRec59[0] = ((fTemp143 * fTemp141) - (((self.fRec59[1] * fTemp147) + (self.fRec59[2] * fTemp149)) / fTemp150));
			let mut fTemp151: F32 = ((fTemp146 * ((self.fRec59[1] + (0.5 * self.fRec59[0])) + (0.5 * self.fRec59[2]))) / fTemp150);
			let mut fTemp152: F32 = (*input3 as F32);
			let mut fTemp153: F32 = (fTemp152 * fTemp26);
			self.fVec11[(self.IOTA & 131071) as usize] = fTemp153;
			let mut fTemp154: F32 = (fTemp119 + (0.0399999991 * ((self.fRec19[0] * fTemp43) * ((fTemp152 * self.fRec20[0]) + (fTemp26 * ((fTemp34 * ((self.fVec11[((self.IOTA - iTemp29) & 131071) as usize] * fTemp31) + (fTemp32 * self.fVec11[((self.IOTA - iTemp33) & 131071) as usize]))) + (fTemp42 * ((self.fVec11[((self.IOTA - iTemp37) & 131071) as usize] * fTemp39) + (self.fVec11[((self.IOTA - iTemp40) & 131071) as usize] * fTemp41)))))))));
			self.fRec63[0] = ((fTemp20 * fTemp154) - (((fTemp123 * self.fRec63[1]) + (fTemp125 * self.fRec63[2])) / fTemp126));
			let mut fTemp155: F32 = (((fTemp20 * (((self.fRec63[0] * fTemp128) + (fTemp127 * self.fRec63[1])) + (fTemp128 * self.fRec63[2]))) / fTemp126) + (self.fRec16[0] * fTemp154));
			let mut fTemp156: F32 = (fTemp131 * self.fRec62[1]);
			self.fRec62[0] = ((fTemp16 * fTemp155) - ((fTemp156 + (fTemp136 * self.fRec62[2])) / fTemp137));
			let mut fTemp157: F32 = (((fTemp16 * ((fTemp156 + (self.fRec62[0] * fTemp139)) + (fTemp140 * self.fRec62[2]))) / fTemp137) + (self.fRec12[0] * fTemp155));
			self.fRec61[0] = ((fTemp143 * fTemp157) - (((fTemp147 * self.fRec61[1]) + (fTemp149 * self.fRec61[2])) / fTemp150));
			let mut fTemp158: F32 = ((fTemp146 * ((self.fRec61[1] + (0.5 * self.fRec61[0])) + (0.5 * self.fRec61[2]))) / fTemp150);
			let mut fTemp159: F32 = (self.fRec3[0] * fTemp157);
			let mut fTemp160: F32 = (fTemp158 + fTemp159);
			let mut fTemp161: F32 = (0.0399999991 * self.fRec2[0]);
			let mut fTemp162: F32 = (fTemp151 + fTemp142);
			let mut fTemp163: F32 = if (iTemp5 as i32 != 0) { (fTemp162 * (1.0 - fTemp161)) } else { (fTemp142 + (fTemp151 + (fTemp160 * F32::abs(fTemp161)))) };
			*output0 = ((0.0199999996 * (self.fRec0[0] * fTemp163)) as F32);
			let mut fTemp164: F32 = if (iTemp5 as i32 != 0) { (fTemp158 + (fTemp159 + (0.0399999991 * (self.fRec2[0] * fTemp162)))) } else { (fTemp160 * (fTemp161 + 1.0)) };
			*output1 = ((0.0199999996 * (self.fRec0[0] * fTemp164)) as F32);
			self.fRec64[0] = ((self.fConst1 * (fTemp3 * self.fRec64[1])) + (fSlow44 * fTemp4));
			self.fRec67[0] = ((self.fConst1 * (fTemp3 * self.fRec67[1])) + (fSlow45 * fTemp4));
			let mut fTemp165: F32 = F32::abs(F32::min(1.0, (0.0399999991 * ((*input0 as F32) * self.fRec67[0]))));
			let mut fTemp166: F32 = if (((self.fRec65[1] > fTemp165) as i32) as i32 != 0) { self.fConst11 } else { 0.0 };
			self.fRec66[0] = ((self.fRec66[1] * fTemp166) + (fTemp165 * (1.0 - fTemp166)));
			self.fRec65[0] = self.fRec66[0];
			let mut fTemp167: F32 = (1.0 - self.fRec65[0]);
			*output2 = ((0.0199999996 * ((self.fRec64[0] * fTemp167) * fTemp163)) as F32);
			let mut fTemp168: F32 = F32::abs(F32::min(1.0, (0.0399999991 * ((*input1 as F32) * self.fRec67[0]))));
			let mut fTemp169: F32 = if (((self.fRec68[1] > fTemp168) as i32) as i32 != 0) { self.fConst11 } else { 0.0 };
			self.fRec69[0] = ((self.fRec69[1] * fTemp169) + (fTemp168 * (1.0 - fTemp169)));
			self.fRec68[0] = self.fRec69[0];
			let mut fTemp170: F32 = (1.0 - self.fRec68[0]);
			*output3 = ((0.0199999996 * ((self.fRec64[0] * fTemp170) * fTemp164)) as F32);
			self.fRec70[0] = ((self.fConst1 * (fTemp3 * self.fRec70[1])) + (fSlow46 * fTemp4));
			*output4 = ((0.0199999996 * ((self.fRec70[0] * fTemp167) * fTemp163)) as F32);
			*output5 = ((0.0199999996 * ((self.fRec70[0] * fTemp170) * fTemp164)) as F32);
			self.fRec71[0] = ((self.fConst1 * (fTemp3 * self.fRec71[1])) + (fSlow47 * fTemp4));
			*output6 = ((0.0199999996 * ((self.fRec71[0] * fTemp167) * fTemp163)) as F32);
			*output7 = ((0.0199999996 * ((self.fRec71[0] * fTemp170) * fTemp164)) as F32);
			self.fVec0[1] = self.fVec0[0];
			self.iVec1[1] = self.iVec1[0];
			self.iRec1[1] = self.iRec1[0];
			self.iVec2[1] = self.iVec2[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[1] = self.fRec9[0];
			self.iRec10[1] = self.iRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec20[1] = self.fRec20[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec22[1] = self.fRec22[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec31[1] = self.fRec31[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec34[1] = self.fRec34[0];
			self.iRec35[1] = self.iRec35[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec43[1] = self.fRec43[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec54[1] = self.fRec54[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec18[2] = self.fRec18[1];
			self.fRec18[1] = self.fRec18[0];
			self.fRec15[2] = self.fRec15[1];
			self.fRec15[1] = self.fRec15[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec59[2] = self.fRec59[1];
			self.fRec59[1] = self.fRec59[0];
			self.fRec63[2] = self.fRec63[1];
			self.fRec63[1] = self.fRec63[0];
			self.fRec62[2] = self.fRec62[1];
			self.fRec62[1] = self.fRec62[0];
			self.fRec61[2] = self.fRec61[1];
			self.fRec61[1] = self.fRec61[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
		}
	}

}

