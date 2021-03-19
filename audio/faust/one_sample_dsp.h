struct one_sample_dsp : dsp {
    struct get_nentries : UI {
        int voice;
        Destinations* destinations;

        get_nentries(Destinations* destinations) : destinations(destinations) {}

        void openTabBox(const char* label) override {
            voice = std::stoi(label);
        }

        void addNumEntry(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {
            destinations->add(voice, std::string(label), zone, min, max);
        }

        void openHorizontalBox(const char* label) override {}
        void openVerticalBox(const char* label) override {}
        void closeBox() override {}
        void addButton(const char* label, FAUSTFLOAT* zone) override {}
        void addCheckButton(const char* label, FAUSTFLOAT* zone) override {}
        void addVerticalSlider(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {}
        void addHorizontalSlider(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {}
        void addHorizontalBargraph(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT min, FAUSTFLOAT max) override {}
        void addVerticalBargraph(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT min, FAUSTFLOAT max) override {}
        void addSoundfile(const char* label, const char* filename, Soundfile** sf_zone) override {}
        void declare(FAUSTFLOAT*, const char*, const char*) override {}
    };

    void prepare(Destinations* destinations) {
        init(SAMPLE_RATE);
        intControls = std::unique_ptr<int[]>(new int[getNumIntControls()]);
        floatControls = std::unique_ptr<float[]>(new float[getNumRealControls()]);
        get_nentries ui { destinations };
        buildUserInterface(&ui);
    }

    void render(FAUSTFLOAT* inputs, FAUSTFLOAT* outputs) {
        control(intControls.get(), floatControls.get());
        compute(inputs, outputs, intControls.get(), floatControls.get());
    }

private:
    std::unique_ptr<int[]> intControls;
    std::unique_ptr<float[]> floatControls;

    virtual int getNumIntControls() = 0;
    virtual int getNumRealControls() = 0;
    virtual void control(int* iControl, FAUSTFLOAT* fControl) = 0;
    virtual void compute(FAUSTFLOAT* inputs, FAUSTFLOAT* outputs, int* iControl, FAUSTFLOAT* fControl) = 0;

    void compute(int, FAUSTFLOAT**, FAUSTFLOAT**) override {}
};
