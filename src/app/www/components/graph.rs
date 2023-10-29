use maud::{html, Markup, Render};

use crate::app::www::i18n::I18n;

pub(crate) struct Dataset {
    label: String,
    data: Vec<(i32, i32, String)>,
}

impl Dataset {
    pub fn new(label: &str, data: Vec<(i32, i32, String)>) -> Self {
        Self {
            label: String::from(label.replace("'", "\\'")),
            data: data
                .into_iter()
                .map(|(a, b, s)| (a, b, s.replace("'", "\\'")))
                .collect(),
        }
    }
}

pub(crate) struct Graph {
    id: String,
    kind: String,
    datasets: Vec<Dataset>,
    xlabel: String,
    ylabel: String,
    title: String,
    subtitle: Option<String>,
}

impl Graph {
    pub fn new(
        id: &str,
        kind: &str,
        mut datasets: Vec<Dataset>,
        xlabel: &str,
        ylabel: &str,
        title: &str,
        subtitle: Option<&str>,
    ) -> Self {
        // Order datasets by decreasing size for nicer rendering.
        datasets.sort_by_key(|ds| usize::MAX - ds.data.len());
        Self {
            id: String::from(id),
            kind: String::from(kind),
            datasets,
            xlabel: xlabel.replace("'", "\\'"),
            ylabel: ylabel.replace("'", "\\'"),
            title: title.replace("'", "\\'"),
            subtitle: subtitle.map(|s| s.replace("'", "\\'")),
        }
    }
}

impl Render for Graph {
    fn render(&self) -> Markup {
        html! {
            canvas #(self.id) style="
            padding: 20px; 
            margin: auto;
            display: block;
            width: 95%;" {};
            script {
                "Chart.defaults.color = '#000';"
                "Chart.defaults.font.size = 16;"
                (format!("var ctx = document.getElementById('{}');", self.id))
                "var data = {
                    datasets: ["
                    @for (i, ds) in self.datasets.iter().enumerate() {
                        (format!("{{ label: '{}',", ds.label))
                            "data: ["
                            @for (x, y, s) in &ds.data {
                                "{ x: " (x) ", y:" (y) ", data: '" (s) "', },"
                            }
                            "],"
                            "
                            backgroundColor: '" (get_color(i))"',
                            borderColor: '#222'
                        },"
                    }
                    "]
                };"
                {
                    "var config = {
                        type: '" (self.kind) "',
                        data: data,
                        options: {
                            scales: {
                                x: {
                                    type: 'linear',
                                    position: 'bottom',
                                    title: {
                                        display: true,
                                        text: '" (self.xlabel) "'
                                    }
                                },
                                y: {
                                    title: {
                                        display: true,
                                        text: '" (self.ylabel) "'
                                    }
                                }
                            },
                            elements: {
                                point: {
                                    radius: 6,
                                    hoverRadius: 8,
                                }
                            },
                            plugins: {
                                title: {
                                    display: true,
                                    text: '" (self.title) "'
                                },"
                                @if let Some(s) = &self.subtitle {
                                    "subtitle: {
                                        display: true,
                                        text: '" (s) "'
                                    },"    
                                }
                                "
                                tooltip: {
                                        callbacks: {
                                        label: function(ctx) {
                                            let label = '';
                                            label += ctx.dataset.label;
                                            label += ': ' + ctx.formattedValue;
                                            label += ' -- ' + ctx.raw.data;
                                            return label;
                                        }
                                    }
                                },
                                "
                                "
                            },
                        }
                    };
                
                    new Chart(ctx, config);"
                }
            }
        }
    }
}

pub(crate) fn view(min_size: usize, graphs: Vec<Graph>) -> Markup {
    html! {
        (make_filters(min_size))
        script src="https://cdn.jsdelivr.net/npm/chart.js" {};
        div {
            @for g in graphs {
                (g)
            }
        }
    }
}

const COLORS: &[&str] = &[
    "#1f78b4", "#b2df8a", "#33a02c", "#fb9a99", "#e31a1c", "#fdbf6f", "#ff7f00", "#cab2d6",
    "#6a3d9a", "#ffff99", "#b15928", "#a6cee3",
];
fn get_color(i: usize) -> &'static str {
    &COLORS[i % COLORS.len()]
}

fn make_filters(min_size: usize) -> Markup {
    html! {
        div style="
            display: flex; justify-content: flex-end;
            "
        {
            form style="
                padding: 0 20px;
                max-width: 500px;
                height: fit-content;
                border-bottom: 2px solid rgb(0, 0, 0);
                border-left: 2px solid rgb(0, 0, 0);
                border-radius: 4px;
                "
            {
                h2 style="
                    font-size: 16px;
                    padding: 10px 32px;
                    margin: 0;
                    font-weight: bold;
                    text-align: center;" 
                    { (I18n::GraphFilters.translate()) }
                label for="min_size" { (I18n::MinDataPointsFilter.translate()) ": " }
                input #min_size type="number" min="0" name="min_size" value=(min_size)
                    style="width: 100px; height: 1.5em;" { }
                br;
                input type="submit" value=(I18n::DoFilter.translate())
                    style="float: right; margin: 5px 0 10px 0;" {}
            }
        }
    }
}
