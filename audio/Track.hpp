struct Track {
    static const int viewsPerPage = 4;

    enum View {
        View_none,
        View_empty,
        View_exactlyOnStep,
        View_containsSteps,
    };

    struct Step {
        bool active;
        bool skipNext;
        int note = 69;
    };

    int resolution = 4;
    int octave = 4;

    Track(int i, mydsp_poly* d, Transport* t, EntryMap e) : id(i), dsp(d), transport(t), entryMap(e), steps() {
    }

    EntryMap::Entry* entry(const std::string& name) {
        return entryMap.contents.count(name) ? &entryMap.contents[name] : nullptr;
    }

    int control(const std::string& name) {
        return entryMap.contents.count(name) ? entryMap.contents[name].value : 0;
    }

    void advance() {
        if (transport->newStep()) {
            auto& step = steps[transport->step % length];
            if (step.skipNext)
                step.skipNext = false;
            else if (step.active)
                keyDown(step.note);
        }
    }

    int bars() {
        return std::ceil(1.f * length / Transport::maxResolution);
    }

    int view(int i) {
        return viewFrom(viewIndexToStart(i));
    }

    int viewStart() {
        return pageStart / viewLength();
    }

    void movePage(int diff) {
        auto newPageStart = pageStart + diff * viewsPerPage * viewLength();
        if (newPageStart < length)
            pageStart = (std::max)(0, newPageStart);
    }

    void zoomOut() {
        if (resolution > 1) {
            resolution /= 2;
            pageStart = pageStart / viewLength() * viewLength();
        }
    }

    void zoomIn() {
        if (resolution < Transport::maxResolution)
            resolution *= 2;
    }

    void toggle(int i) {
        auto start = viewIndexToStart(i);
        switch (viewFrom(start)) {
            case View_none:
                return;
            case View_empty:
            case View_exactlyOnStep:
                steps[start].active ^= true;
                steps[start].skipNext = false;
                return;
            case View_containsSteps:
                for (int i = start; i < start + viewLength(); i++)
                    steps[i].active = false;
                return;
        }
    }

    void play() {
        play(entryMap.contents["naturalNote"].value);
    }

    void play(int note) {
        if (transport->playing && transport->armed) {
            auto quantizedStep = transport->quantizedStep(resolution);
            steps[quantizedStep % length] = {
                .active = true,
                .skipNext = quantizedStep >= transport->step,
                .note = note 
            };
        }
        keyDown(note);
    }

    void release() {
        release(entryMap.contents["naturalNote"].value);
    }

    void release(int note) {
        dsp->keyOff(id, note);
    }

  private:
    int id;
    int pageStart = 0;
    int length = Transport::maxResolution*4;
    mydsp_poly* dsp;
    Transport* transport;
    EntryMap entryMap;
    std::array<Step, Transport::maxResolution*16*8> steps;

    int viewLength() {
        return Transport::maxResolution / resolution;
    }

    int viewIndexToStart(int i) {
        return pageStart + i * viewLength();
    }

    View viewFrom(int start) {
        if (start >= length)
            return View_none;
        int countActive = 0;
        int lastActive = 0;
        for (int i = start; i < start + viewLength(); i++) {
            if (steps[i].active) {
                countActive++;
                lastActive = i;
            }
        }
        if (countActive == 0)
            return View_empty;
        else if (countActive == 1 && lastActive == start)
            return View_exactlyOnStep;
        else
            return View_containsSteps;
    }

    void keyDown(int note) {
        auto ui = dsp->keyOn(id, note, 127);
        ui->setParamValue("sampleFile", id);
        for (const auto& pair : entryMap.contents)
            ui->setParamValue(pair.first, pair.second.value);
    }
};
