
pub struct echo {
	fSampleRate: i32,
	fConst1: F32,
	fEntry0: F32,
	fConst2: F32,
	fRec0: [F32;2],
	IOTA: i32,
	fVec0: [F32;4194304],
	fEntry1: F32,
	fConst3: F32,
	fConst4: F32,
	fEntry2: F32,
	fRec1: [F32;524288],
	fRec2: [F32;2],
	iRec3: [i32;2],
	fVec1: [F32;4194304],
	fRec4: [F32;524288],
}

impl FaustDsp for echo {
	type T = F32;
		
	fn new() -> echo { 
		echo {
			fSampleRate: 0,
			fConst1: 0.0,
			fEntry0: 0.0,
			fConst2: 0.0,
			fRec0: [0.0;2],
			IOTA: 0,
			fVec0: [0.0;4194304],
			fEntry1: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fEntry2: 0.0,
			fRec1: [0.0;524288],
			fRec2: [0.0;2],
			iRec3: [0;2],
			fVec1: [0.0;4194304],
			fRec4: [0.0;524288],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("filename", "echo.dsp");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("misceffects.lib/name", "Misc Effects Library");
		m.declare("misceffects.lib/version", "2.0");
		m.declare("name", "echo");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("routes.lib/name", "Faust Signal Routing Library");
		m.declare("routes.lib/version", "0.2");
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
			self.fRec0[l0 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l1 in 0..4194304 {
			self.fVec0[l1 as usize] = 0.0;
		}
		for l2 in 0..524288 {
			self.fRec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec2[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.iRec3[l4 as usize] = 0;
		}
		for l5 in 0..4194304 {
			self.fVec1[l5 as usize] = 0.0;
		}
		for l6 in 0..524288 {
			self.fRec4[l6 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		let mut fConst0: F32 = F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)));
		self.fConst1 = (44.0999985 / fConst0);
		self.fConst2 = (1.0 - self.fConst1);
		self.fConst3 = (2.0 * fConst0);
		self.fConst4 = (0.0399999991 * fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		echo::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("echo");
		ui_interface.add_num_entry("echoFeed", ParamIndex(0), 25.0, -1.0, 50.0, 10.0);
		ui_interface.add_num_entry("echoGain", ParamIndex(1), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("echoSpace", ParamIndex(2), 25.0, 0.0, 50.0, 10.0);
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
		let mut fSlow0: F32 = (self.fConst1 * (self.fEntry0 as F32));
		let mut fSlow1: F32 = (self.fEntry1 as F32);
		let mut fSlow2: F32 = (0.0196078438 * fSlow1);
		let mut fSlow3: F32 = (self.fEntry2 as F32);
		let mut iSlow4: i32 = ((F32::min(self.fConst3, F32::max(0.0, (self.fConst4 * fSlow3))) as i32) + 1);
		let mut fSlow5: F32 = (((((fSlow1 == -1.0) as i32) >= 1) as i32) as F32);
		let mut iSlow6: i32 = (F32::powf(2.0, ((((0.200000003 * fSlow3) as i32) + 12) as F32)) as i32);
		let mut fSlow7: F32 = (iSlow6 as F32);
		let mut fSlow8: F32 = (4.0 / fSlow7);
		let mut iSlow9: i32 = (iSlow6 + -1);
		let mut fSlow10: F32 = (1.0 / fSlow7);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec0[0] = (fSlow0 + (self.fConst2 * self.fRec0[1]));
			let mut fTemp0: F32 = (*input0 as F32);
			self.fVec0[(self.IOTA & 4194303) as usize] = fTemp0;
			self.fRec1[(self.IOTA & 524287) as usize] = (fTemp0 + (fSlow2 * self.fRec1[((self.IOTA - iSlow4) & 524287) as usize]));
			let mut fTemp1: F32 = (self.fRec2[1] + 1.0);
			let mut fTemp2: F32 = (self.fRec2[1] + -1.0);
			self.fRec2[0] = if (((fTemp1 < fSlow5) as i32) as i32 != 0) { fTemp1 } else { if (((fTemp2 > fSlow5) as i32) as i32 != 0) { fTemp2 } else { fSlow5 } };
			let mut fTemp3: F32 = (1.0 - self.fRec2[0]);
			self.iRec3[0] = (self.iRec3[1] + 2);
			let mut iTemp4: i32 = (self.iRec3[0] & iSlow9);
			let mut fTemp5: F32 = (iTemp4 as F32);
			let mut fTemp6: F32 = ((self.fRec2[0] * fTemp5) * (1.0 - (fSlow10 * fTemp5)));
			let mut iTemp7: i32 = std::cmp::min(iSlow6, std::cmp::max(0, iTemp4));
			*output0 = ((0.100000001 * (self.fRec0[0] * ((self.fRec1[((self.IOTA - 0) & 524287) as usize] * fTemp3) + (fSlow8 * (fTemp6 * self.fVec0[((self.IOTA - iTemp7) & 4194303) as usize]))))) as F32);
			let mut fTemp8: F32 = (*input1 as F32);
			self.fVec1[(self.IOTA & 4194303) as usize] = fTemp8;
			self.fRec4[(self.IOTA & 524287) as usize] = (fTemp8 + (fSlow2 * self.fRec4[((self.IOTA - iSlow4) & 524287) as usize]));
			*output1 = ((0.100000001 * (self.fRec0[0] * ((self.fRec4[((self.IOTA - 0) & 524287) as usize] * fTemp3) + (fSlow8 * (fTemp6 * self.fVec1[((self.IOTA - iTemp7) & 4194303) as usize]))))) as F32);
			self.fRec0[1] = self.fRec0[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec2[1] = self.fRec2[0];
			self.iRec3[1] = self.iRec3[0];
		}
	}

}

