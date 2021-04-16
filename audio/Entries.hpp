struct Entries : GenericUI {
    struct Control {
        const char* label;
        float value;
        float min;
        float max;
        float step;
    };

    std::vector<Control> data;

    Entries() : data() {
    }

    void addNumEntry(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {
        if (writeIndex < 0)
            data.push_back({ label, init, min, max, step });
        else
            *zone = data[writeIndex++].value;
    }

    void prepareToWrite() {
        writeIndex = 0;
    }

  private:
    int writeIndex = -1;
};
