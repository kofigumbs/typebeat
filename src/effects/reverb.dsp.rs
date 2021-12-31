
pub struct reverb {
	fEntry0: F32,
	fRec9: [F32;2],
	fSampleRate: i32,
	fConst1: F32,
	fEntry1: F32,
	fConst2: F32,
	fRec10: [F32;2],
	fEntry2: F32,
	fRec11: [F32;2],
	IOTA: i32,
	fVec0: [F32;8192],
	iConst3: i32,
	fRec8: [F32;2],
	fRec13: [F32;2],
	fVec1: [F32;8192],
	iConst4: i32,
	fRec12: [F32;2],
	fRec15: [F32;2],
	fVec2: [F32;8192],
	iConst5: i32,
	fRec14: [F32;2],
	fRec17: [F32;2],
	fVec3: [F32;8192],
	iConst6: i32,
	fRec16: [F32;2],
	fRec19: [F32;2],
	fVec4: [F32;8192],
	iConst7: i32,
	fRec18: [F32;2],
	fRec21: [F32;2],
	fVec5: [F32;8192],
	iConst8: i32,
	fRec20: [F32;2],
	fRec23: [F32;2],
	fVec6: [F32;8192],
	iConst9: i32,
	fRec22: [F32;2],
	fRec25: [F32;2],
	fVec7: [F32;8192],
	iConst10: i32,
	fRec24: [F32;2],
	fVec8: [F32;2048],
	iConst12: i32,
	fRec6: [F32;2],
	fVec9: [F32;2048],
	iConst14: i32,
	fRec4: [F32;2],
	fVec10: [F32;2048],
	iConst16: i32,
	fRec2: [F32;2],
	fVec11: [F32;1024],
	iConst18: i32,
	fRec0: [F32;2],
	fRec35: [F32;2],
	fVec12: [F32;8192],
	iConst19: i32,
	fRec34: [F32;2],
	fRec37: [F32;2],
	fVec13: [F32;8192],
	iConst20: i32,
	fRec36: [F32;2],
	fRec39: [F32;2],
	fVec14: [F32;8192],
	iConst21: i32,
	fRec38: [F32;2],
	fRec41: [F32;2],
	fVec15: [F32;8192],
	iConst22: i32,
	fRec40: [F32;2],
	fRec43: [F32;2],
	fVec16: [F32;8192],
	iConst23: i32,
	fRec42: [F32;2],
	fRec45: [F32;2],
	fVec17: [F32;8192],
	iConst24: i32,
	fRec44: [F32;2],
	fRec47: [F32;2],
	fVec18: [F32;8192],
	iConst25: i32,
	fRec46: [F32;2],
	fRec49: [F32;2],
	fVec19: [F32;8192],
	iConst26: i32,
	fRec48: [F32;2],
	fVec20: [F32;2048],
	iConst27: i32,
	fRec32: [F32;2],
	fVec21: [F32;2048],
	iConst28: i32,
	fRec30: [F32;2],
	fVec22: [F32;2048],
	iConst29: i32,
	fRec28: [F32;2],
	fVec23: [F32;1024],
	iConst30: i32,
	fRec26: [F32;2],
}

impl FaustDsp for reverb {
	type T = F32;
		
	fn new() -> reverb { 
		reverb {
			fEntry0: 0.0,
			fRec9: [0.0;2],
			fSampleRate: 0,
			fConst1: 0.0,
			fEntry1: 0.0,
			fConst2: 0.0,
			fRec10: [0.0;2],
			fEntry2: 0.0,
			fRec11: [0.0;2],
			IOTA: 0,
			fVec0: [0.0;8192],
			iConst3: 0,
			fRec8: [0.0;2],
			fRec13: [0.0;2],
			fVec1: [0.0;8192],
			iConst4: 0,
			fRec12: [0.0;2],
			fRec15: [0.0;2],
			fVec2: [0.0;8192],
			iConst5: 0,
			fRec14: [0.0;2],
			fRec17: [0.0;2],
			fVec3: [0.0;8192],
			iConst6: 0,
			fRec16: [0.0;2],
			fRec19: [0.0;2],
			fVec4: [0.0;8192],
			iConst7: 0,
			fRec18: [0.0;2],
			fRec21: [0.0;2],
			fVec5: [0.0;8192],
			iConst8: 0,
			fRec20: [0.0;2],
			fRec23: [0.0;2],
			fVec6: [0.0;8192],
			iConst9: 0,
			fRec22: [0.0;2],
			fRec25: [0.0;2],
			fVec7: [0.0;8192],
			iConst10: 0,
			fRec24: [0.0;2],
			fVec8: [0.0;2048],
			iConst12: 0,
			fRec6: [0.0;2],
			fVec9: [0.0;2048],
			iConst14: 0,
			fRec4: [0.0;2],
			fVec10: [0.0;2048],
			iConst16: 0,
			fRec2: [0.0;2],
			fVec11: [0.0;1024],
			iConst18: 0,
			fRec0: [0.0;2],
			fRec35: [0.0;2],
			fVec12: [0.0;8192],
			iConst19: 0,
			fRec34: [0.0;2],
			fRec37: [0.0;2],
			fVec13: [0.0;8192],
			iConst20: 0,
			fRec36: [0.0;2],
			fRec39: [0.0;2],
			fVec14: [0.0;8192],
			iConst21: 0,
			fRec38: [0.0;2],
			fRec41: [0.0;2],
			fVec15: [0.0;8192],
			iConst22: 0,
			fRec40: [0.0;2],
			fRec43: [0.0;2],
			fVec16: [0.0;8192],
			iConst23: 0,
			fRec42: [0.0;2],
			fRec45: [0.0;2],
			fVec17: [0.0;8192],
			iConst24: 0,
			fRec44: [0.0;2],
			fRec47: [0.0;2],
			fVec18: [0.0;8192],
			iConst25: 0,
			fRec46: [0.0;2],
			fRec49: [0.0;2],
			fVec19: [0.0;8192],
			iConst26: 0,
			fRec48: [0.0;2],
			fVec20: [0.0;2048],
			iConst27: 0,
			fRec32: [0.0;2],
			fVec21: [0.0;2048],
			iConst28: 0,
			fRec30: [0.0;2],
			fVec22: [0.0;2048],
			iConst29: 0,
			fRec28: [0.0;2],
			fVec23: [0.0;1024],
			iConst30: 0,
			fRec26: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("filename", "reverb.dsp");
		m.declare("filters.lib/allpass_comb:author", "Julius O. Smith III");
		m.declare("filters.lib/allpass_comb:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpass_comb:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "MIT-style STK-4.3 license");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/version", "0.3");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("name", "reverb");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("reverbs.lib/name", "Faust Reverb Library");
		m.declare("reverbs.lib/version", "0.2");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("spats.lib/name", "Faust Spatialization Library");
		m.declare("spats.lib/version", "0.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fEntry0 = 25.0;
		self.fEntry1 = 25.0;
		self.fEntry2 = 25.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec9[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec10[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec11[l2 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l3 in 0..8192 {
			self.fVec0[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec8[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec13[l5 as usize] = 0.0;
		}
		for l6 in 0..8192 {
			self.fVec1[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec12[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec15[l8 as usize] = 0.0;
		}
		for l9 in 0..8192 {
			self.fVec2[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec14[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec17[l11 as usize] = 0.0;
		}
		for l12 in 0..8192 {
			self.fVec3[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec16[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec19[l14 as usize] = 0.0;
		}
		for l15 in 0..8192 {
			self.fVec4[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec18[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec21[l17 as usize] = 0.0;
		}
		for l18 in 0..8192 {
			self.fVec5[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec20[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec23[l20 as usize] = 0.0;
		}
		for l21 in 0..8192 {
			self.fVec6[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec22[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec25[l23 as usize] = 0.0;
		}
		for l24 in 0..8192 {
			self.fVec7[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec24[l25 as usize] = 0.0;
		}
		for l26 in 0..2048 {
			self.fVec8[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec6[l27 as usize] = 0.0;
		}
		for l28 in 0..2048 {
			self.fVec9[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec4[l29 as usize] = 0.0;
		}
		for l30 in 0..2048 {
			self.fVec10[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec2[l31 as usize] = 0.0;
		}
		for l32 in 0..1024 {
			self.fVec11[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec0[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec35[l34 as usize] = 0.0;
		}
		for l35 in 0..8192 {
			self.fVec12[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec34[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec37[l37 as usize] = 0.0;
		}
		for l38 in 0..8192 {
			self.fVec13[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec36[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec39[l40 as usize] = 0.0;
		}
		for l41 in 0..8192 {
			self.fVec14[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec38[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec41[l43 as usize] = 0.0;
		}
		for l44 in 0..8192 {
			self.fVec15[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec40[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec43[l46 as usize] = 0.0;
		}
		for l47 in 0..8192 {
			self.fVec16[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec42[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec45[l49 as usize] = 0.0;
		}
		for l50 in 0..8192 {
			self.fVec17[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec44[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec47[l52 as usize] = 0.0;
		}
		for l53 in 0..8192 {
			self.fVec18[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec46[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec49[l55 as usize] = 0.0;
		}
		for l56 in 0..8192 {
			self.fVec19[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec48[l57 as usize] = 0.0;
		}
		for l58 in 0..2048 {
			self.fVec20[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec32[l59 as usize] = 0.0;
		}
		for l60 in 0..2048 {
			self.fVec21[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec30[l61 as usize] = 0.0;
		}
		for l62 in 0..2048 {
			self.fVec22[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec28[l63 as usize] = 0.0;
		}
		for l64 in 0..1024 {
			self.fVec23[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec26[l65 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)));
		self.fConst1 = (44.0999985 / fConst0);
		self.fConst2 = (1.0 - self.fConst1);
		self.iConst3 = ((0.0253061224 * fConst0) as i32);
		self.iConst4 = ((0.0322448984 * fConst0) as i32);
		self.iConst5 = ((0.0307482984 * fConst0) as i32);
		self.iConst6 = ((0.0289569162 * fConst0) as i32);
		self.iConst7 = ((0.0269387756 * fConst0) as i32);
		self.iConst8 = ((0.0366666652 * fConst0) as i32);
		self.iConst9 = ((0.0353061222 * fConst0) as i32);
		self.iConst10 = ((0.033809524 * fConst0) as i32);
		let mut iConst11: i32 = ((0.0126077095 * fConst0) as i32);
		self.iConst12 = std::cmp::min(1024, std::cmp::max(0, (iConst11 + -1)));
		let mut iConst13: i32 = ((0.00999999978 * fConst0) as i32);
		self.iConst14 = std::cmp::min(1024, std::cmp::max(0, (iConst13 + -1)));
		let mut iConst15: i32 = ((0.00773242628 * fConst0) as i32);
		self.iConst16 = std::cmp::min(1024, std::cmp::max(0, (iConst15 + -1)));
		let mut iConst17: i32 = ((0.00510204071 * fConst0) as i32);
		self.iConst18 = std::cmp::min(1024, std::cmp::max(0, (iConst17 + -1)));
		self.iConst19 = (self.iConst4 + 23);
		self.iConst20 = (self.iConst5 + 23);
		self.iConst21 = (self.iConst6 + 23);
		self.iConst22 = (self.iConst7 + 23);
		self.iConst23 = (self.iConst8 + 23);
		self.iConst24 = (self.iConst9 + 23);
		self.iConst25 = (self.iConst10 + 23);
		self.iConst26 = (self.iConst3 + 23);
		self.iConst27 = std::cmp::min(1024, std::cmp::max(0, (iConst11 + 22)));
		self.iConst28 = std::cmp::min(1024, std::cmp::max(0, (iConst13 + 22)));
		self.iConst29 = std::cmp::min(1024, std::cmp::max(0, (iConst15 + 22)));
		self.iConst30 = std::cmp::min(1024, std::cmp::max(0, (iConst17 + 22)));
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		reverb::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("reverb");
		ui_interface.add_num_entry("reverbComb", ParamIndex(0), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("reverbDamp", ParamIndex(1), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("reverbGain", ParamIndex(2), 25.0, 0.0, 50.0, 10.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			1 => Some(self.fEntry0),
			0 => Some(self.fEntry1),
			2 => Some(self.fEntry2),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			1 => { self.fEntry0 = value }
			0 => { self.fEntry1 = value }
			2 => { self.fEntry2 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			(inputs0, inputs1)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = (0.0199999996 * (self.fEntry0 as F32));
		let mut fSlow1: F32 = (1.0 - fSlow0);
		let mut fSlow2: F32 = (self.fConst1 * (self.fEntry1 as F32));
		let mut fSlow3: F32 = (self.fConst1 * (self.fEntry2 as F32));
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec9[0] = ((fSlow0 * self.fRec9[1]) + (fSlow1 * self.fRec8[1]));
			self.fRec10[0] = (fSlow2 + (self.fConst2 * self.fRec10[1]));
			let mut fTemp0: F32 = ((0.00571428565 * self.fRec10[0]) + 0.699999988);
			self.fRec11[0] = (fSlow3 + (self.fConst2 * self.fRec11[1]));
			let mut fTemp1: F32 = (0.00400000019 * (self.fRec11[0] * ((*input0 as F32) + (*input1 as F32))));
			self.fVec0[(self.IOTA & 8191) as usize] = ((self.fRec9[0] * fTemp0) + fTemp1);
			self.fRec8[0] = self.fVec0[((self.IOTA - self.iConst3) & 8191) as usize];
			self.fRec13[0] = ((fSlow0 * self.fRec13[1]) + (fSlow1 * self.fRec12[1]));
			self.fVec1[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec13[0] * fTemp0));
			self.fRec12[0] = self.fVec1[((self.IOTA - self.iConst4) & 8191) as usize];
			self.fRec15[0] = ((fSlow0 * self.fRec15[1]) + (fSlow1 * self.fRec14[1]));
			self.fVec2[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec15[0] * fTemp0));
			self.fRec14[0] = self.fVec2[((self.IOTA - self.iConst5) & 8191) as usize];
			self.fRec17[0] = ((fSlow0 * self.fRec17[1]) + (fSlow1 * self.fRec16[1]));
			self.fVec3[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec17[0] * fTemp0));
			self.fRec16[0] = self.fVec3[((self.IOTA - self.iConst6) & 8191) as usize];
			self.fRec19[0] = ((fSlow0 * self.fRec19[1]) + (fSlow1 * self.fRec18[1]));
			self.fVec4[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec19[0] * fTemp0));
			self.fRec18[0] = self.fVec4[((self.IOTA - self.iConst7) & 8191) as usize];
			self.fRec21[0] = ((fSlow0 * self.fRec21[1]) + (fSlow1 * self.fRec20[1]));
			self.fVec5[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec21[0] * fTemp0));
			self.fRec20[0] = self.fVec5[((self.IOTA - self.iConst8) & 8191) as usize];
			self.fRec23[0] = ((fSlow0 * self.fRec23[1]) + (fSlow1 * self.fRec22[1]));
			self.fVec6[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec23[0] * fTemp0));
			self.fRec22[0] = self.fVec6[((self.IOTA - self.iConst9) & 8191) as usize];
			self.fRec25[0] = ((fSlow0 * self.fRec25[1]) + (fSlow1 * self.fRec24[1]));
			self.fVec7[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec25[0] * fTemp0));
			self.fRec24[0] = self.fVec7[((self.IOTA - self.iConst10) & 8191) as usize];
			let mut fTemp2: F32 = (self.fRec8[0] + (self.fRec12[0] + (self.fRec14[0] + (self.fRec16[0] + (self.fRec18[0] + (self.fRec20[0] + (self.fRec22[0] + (self.fRec24[0] + (0.5 * self.fRec6[1])))))))));
			self.fVec8[(self.IOTA & 2047) as usize] = fTemp2;
			self.fRec6[0] = self.fVec8[((self.IOTA - self.iConst12) & 2047) as usize];
			let mut fRec7: F32 = (0.0 - (0.5 * fTemp2));
			let mut fTemp3: F32 = (fRec7 + ((0.5 * self.fRec4[1]) + self.fRec6[1]));
			self.fVec9[(self.IOTA & 2047) as usize] = fTemp3;
			self.fRec4[0] = self.fVec9[((self.IOTA - self.iConst14) & 2047) as usize];
			let mut fRec5: F32 = (0.0 - (0.5 * fTemp3));
			let mut fTemp4: F32 = (fRec5 + ((0.5 * self.fRec2[1]) + self.fRec4[1]));
			self.fVec10[(self.IOTA & 2047) as usize] = fTemp4;
			self.fRec2[0] = self.fVec10[((self.IOTA - self.iConst16) & 2047) as usize];
			let mut fRec3: F32 = (0.0 - (0.5 * fTemp4));
			let mut fTemp5: F32 = (fRec3 + ((0.5 * self.fRec0[1]) + self.fRec2[1]));
			self.fVec11[(self.IOTA & 1023) as usize] = fTemp5;
			self.fRec0[0] = self.fVec11[((self.IOTA - self.iConst18) & 1023) as usize];
			let mut fRec1: F32 = (0.0 - (0.5 * fTemp5));
			*output0 = ((fRec1 + self.fRec0[1]) as F32);
			self.fRec35[0] = ((fSlow0 * self.fRec35[1]) + (fSlow1 * self.fRec34[1]));
			self.fVec12[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec35[0] * fTemp0));
			self.fRec34[0] = self.fVec12[((self.IOTA - self.iConst19) & 8191) as usize];
			self.fRec37[0] = ((fSlow0 * self.fRec37[1]) + (fSlow1 * self.fRec36[1]));
			self.fVec13[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec37[0] * fTemp0));
			self.fRec36[0] = self.fVec13[((self.IOTA - self.iConst20) & 8191) as usize];
			self.fRec39[0] = ((fSlow0 * self.fRec39[1]) + (fSlow1 * self.fRec38[1]));
			self.fVec14[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec39[0] * fTemp0));
			self.fRec38[0] = self.fVec14[((self.IOTA - self.iConst21) & 8191) as usize];
			self.fRec41[0] = ((fSlow0 * self.fRec41[1]) + (fSlow1 * self.fRec40[1]));
			self.fVec15[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec41[0] * fTemp0));
			self.fRec40[0] = self.fVec15[((self.IOTA - self.iConst22) & 8191) as usize];
			self.fRec43[0] = ((fSlow0 * self.fRec43[1]) + (fSlow1 * self.fRec42[1]));
			self.fVec16[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec43[0] * fTemp0));
			self.fRec42[0] = self.fVec16[((self.IOTA - self.iConst23) & 8191) as usize];
			self.fRec45[0] = ((fSlow0 * self.fRec45[1]) + (fSlow1 * self.fRec44[1]));
			self.fVec17[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec45[0] * fTemp0));
			self.fRec44[0] = self.fVec17[((self.IOTA - self.iConst24) & 8191) as usize];
			self.fRec47[0] = ((fSlow0 * self.fRec47[1]) + (fSlow1 * self.fRec46[1]));
			self.fVec18[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec47[0] * fTemp0));
			self.fRec46[0] = self.fVec18[((self.IOTA - self.iConst25) & 8191) as usize];
			self.fRec49[0] = ((fSlow0 * self.fRec49[1]) + (fSlow1 * self.fRec48[1]));
			self.fVec19[(self.IOTA & 8191) as usize] = (fTemp1 + (self.fRec49[0] * fTemp0));
			self.fRec48[0] = self.fVec19[((self.IOTA - self.iConst26) & 8191) as usize];
			let mut fTemp6: F32 = (self.fRec34[0] + (self.fRec36[0] + (self.fRec38[0] + (self.fRec40[0] + (self.fRec42[0] + (self.fRec44[0] + (self.fRec46[0] + (self.fRec48[0] + (0.5 * self.fRec32[1])))))))));
			self.fVec20[(self.IOTA & 2047) as usize] = fTemp6;
			self.fRec32[0] = self.fVec20[((self.IOTA - self.iConst27) & 2047) as usize];
			let mut fRec33: F32 = (0.0 - (0.5 * fTemp6));
			let mut fTemp7: F32 = (fRec33 + ((0.5 * self.fRec30[1]) + self.fRec32[1]));
			self.fVec21[(self.IOTA & 2047) as usize] = fTemp7;
			self.fRec30[0] = self.fVec21[((self.IOTA - self.iConst28) & 2047) as usize];
			let mut fRec31: F32 = (0.0 - (0.5 * fTemp7));
			let mut fTemp8: F32 = (fRec31 + ((0.5 * self.fRec28[1]) + self.fRec30[1]));
			self.fVec22[(self.IOTA & 2047) as usize] = fTemp8;
			self.fRec28[0] = self.fVec22[((self.IOTA - self.iConst29) & 2047) as usize];
			let mut fRec29: F32 = (0.0 - (0.5 * fTemp8));
			let mut fTemp9: F32 = (fRec29 + ((0.5 * self.fRec26[1]) + self.fRec28[1]));
			self.fVec23[(self.IOTA & 1023) as usize] = fTemp9;
			self.fRec26[0] = self.fVec23[((self.IOTA - self.iConst30) & 1023) as usize];
			let mut fRec27: F32 = (0.0 - (0.5 * fTemp9));
			*output1 = ((fRec27 + self.fRec26[1]) as F32);
			self.fRec9[1] = self.fRec9[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec8[1] = self.fRec8[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec26[1] = self.fRec26[0];
		}
	}

}

