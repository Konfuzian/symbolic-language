import { useState } from 'react';

const colors = {
  comment: '#6a9955',
  variable: '#c586c0',
  key: '#9cdcfe',
  symbol: '#4ec9b0',
  number: '#b5cea8',
  boolean: '#569cd6',
  null: '#569cd6',
  bracket: '#ffd700',
  escape: '#d7ba7d',
  string: '#ce9178',
  default: '#d4d4d4',
};

function tokenize(code) {
  const tokens = [];
  let i = 0;
  while (i < code.length) {
    if (code.slice(i, i + 2) === '/*') {
      const end = code.indexOf('*/', i + 2);
      const endIdx = end === -1 ? code.length : end + 2;
      tokens.push({ type: 'comment', value: code.slice(i, endIdx) });
      i = endIdx;
      continue;
    }
    if (code.slice(i, i + 2) === '//') {
      const prevChar = i > 0 ? code[i - 1] : '\n';
      if (prevChar === '\n' || prevChar === ' ' || prevChar === '\t' || i === 0) {
        const end = code.indexOf('\n', i);
        tokens.push({ type: 'comment', value: code.slice(i, end === -1 ? code.length : end) });
        i = end === -1 ? code.length : end;
        continue;
      }
    }
    if (code[i] === '\n') { tokens.push({ type: 'newline', value: '\n' }); i++; continue; }
    if ('{['.includes(code[i])) { tokens.push({ type: 'bracket', value: code[i] }); i++; continue; }
    if ('}]'.includes(code[i])) { tokens.push({ type: 'bracket', value: code[i] }); i++; continue; }
    if (code[i] === ',') { tokens.push({ type: 'default', value: ',' }); i++; continue; }
    if (code[i] === '$') {
      let j = i + 1;
      while (j < code.length && /[a-zA-Z0-9_-]/.test(code[j])) j++;
      if (code[j] === '!') j++;
      tokens.push({ type: 'variable', value: code.slice(i, j) });
      i = j;
      continue;
    }
    if (code[i] === ':') {
      let j = i + 1;
      while (j < code.length && /[a-zA-Z0-9_-]/.test(code[j])) j++;
      if (code[j] === '!' || code[j] === '+') j++;
      const lastNonWs = tokens.filter(t => t.type !== 'newline' && t.value.trim()).pop();
      const isKey = !lastNonWs || lastNonWs.value === '{' || lastNonWs.value === ',';
      tokens.push({ type: isKey ? 'key' : 'symbol', value: code.slice(i, j) });
      i = j;
      continue;
    }
    if (code[i] === '\\') {
      let j = i + 1;
      if (j < code.length) { j++; while (j < code.length && /[a-zA-Z0-9_-]/.test(code[j])) j++; }
      tokens.push({ type: 'escape', value: code.slice(i, j) });
      i = j;
      continue;
    }
    if (' \t'.includes(code[i])) {
      let j = i;
      while (j < code.length && ' \t'.includes(code[j])) j++;
      tokens.push({ type: 'default', value: code.slice(i, j) });
      i = j;
      continue;
    }
    let j = i;
    while (j < code.length && !' \t\n{}[],:'.includes(code[j])) j++;
    const word = code.slice(i, j);
    if (word === 'true' || word === 'false') tokens.push({ type: 'boolean', value: word });
    else if (word === 'null') tokens.push({ type: 'null', value: word });
    else if (/^(inf|-inf|nan)$/.test(word)) tokens.push({ type: 'number', value: word });
    else if (/^-?(\d[\d_]*\.?[\d_]*([eE][+-]?\d+)?|0x[\da-fA-F_]+|0b[01_]+|0o[0-7_]+)$/.test(word)) tokens.push({ type: 'number', value: word });
    else tokens.push({ type: 'string', value: word });
    i = j;
  }
  return tokens;
}

function Highlight({ code }) {
  return (
    <pre className="bg-[#1e1e1e] p-4 overflow-auto font-mono text-xs leading-relaxed m-0">
      <code>{tokenize(code).map((t, i) => <span key={i} style={{ color: colors[t.type] || colors.default }}>{t.value}</span>)}</code>
    </pre>
  );
}

const examples = {
  'Simple Config': `// Simple application configuration

{ $app myapp
, $version 1.0.0
, $debug true
}

{ :meta
  { :name $app
  , :version $version
  }

, :server
  { :host localhost
  , :port 8080
  , :ssl false
  , :status :running
  }

, :tags
  [ :api
  , :backend
  , :production
  ]

, :description
    This is a multiline description.
    It can span multiple lines.
    Commas, like this one, are preserved inline.
}`,

  'Docker Compose': `// Docker Compose in SYM

{ $db-name myapp
, $db-user admin
, $db-pass secretpass123
}

{ :version 3.8
, :services
  { :nginx
    { :image nginx:alpine
    , :ports
      [ 80:80
      , 443:443
      ]
    , :restart :always
    }
  , :api
    { :build
      { :context ./api
      }
    , :environment
      { :NODE_ENV production
      , :DATABASE_URL postgres://$db-user:$db-pass@db:5432/$db-name
      }
    , :depends_on
      [ :db
      , :redis
      ]
    }
  , :db
    { :image postgres:16-alpine
    , :environment
      { :POSTGRES_DB $db-name
      , :POSTGRES_USER $db-user
      , :POSTGRES_PASSWORD $db-pass
      }
    }
  }
, :volumes
  { :pgdata
  }
}`,

  'Kubernetes': `// Kubernetes Deployment

{ $app web-api
, $replicas 3
, $image-tag v2.4.1
}

{ :apiVersion apps/v1
, :kind :Deployment
, :metadata
  { :name $app
  , :labels
    { :app $app
    , :env production
    }
  }
, :spec
  { :replicas $replicas
  , :template
    { :spec
      { :containers
        [ { :name $app
          , :image gcr.io/myproject/$app:$image-tag
          , :ports
            [ { :containerPort 8080
              }
            ]
          , :resources
            { :requests
              { :cpu 100m
              , :memory 256Mi
              }
            , :limits
              { :cpu 500m
              , :memory 512Mi
              }
            }
          , :livenessProbe
            { :httpGet
              { :path /health
              , :port 8080
              }
            }
          }
        ]
      }
    }
  }
}`,

  'Package.json': `// Node.js package manifest

{ :name @acme/billing-service
, :version 3.2.0
, :description
    Internal billing service for processing
    payments and generating invoices.
, :license MIT
, :private true

, :scripts
  { :dev nodemon src/index.ts
  , :build tsc
  , :test jest --coverage
  }

, :dependencies
  { :express 4.18.2
  , :stripe 14.10.0
  , :zod 3.22.4
  }

, :devDependencies
  { :typescript 5.3.3
  , :jest 29.7.0
  }

, :engines
  { :node >=20.0.0
  }
}`,

  'GitHub Actions': `// GitHub Actions CI/CD

{ :name CI/CD Pipeline
, :on
  { :push
    { :branches
      [ main
      , develop
      ]
    }
  , :pull_request
    { :branches
      [ main
      ]
    }
  }

, :jobs
  { :test
    { :runs-on ubuntu-latest
    , :services
      { :postgres
        { :image postgres:16-alpine
        , :env
          { :POSTGRES_USER test
          , :POSTGRES_PASSWORD test
          }
        }
      }
    , :steps
      [ { :uses actions/checkout@v4
        }
      , { :uses actions/setup-node@v4
        , :with
          { :node-version 20
          }
        }
      , { :run npm ci
        }
      , { :run npm test
        }
      ]
    }
  , :deploy
    { :needs test
    , :if github.ref == 'refs/heads/main'
    , :runs-on ubuntu-latest
    , :steps
      [ { :run echo "Deploying..."
        }
      ]
    }
  }
}`,

  'App Config': `// Production app configuration

{ $env production
, $region us-east-1
}

{ :app
  { :name acme-platform
  , :version 4.12.0
  }

, :server
  { :host 0.0.0.0
  , :port 8080
  , :cors
    { :origins
      [ https://app.acme.com
      , https://admin.acme.com
      ]
    }
  }

, :database
  { :host db.$region.acme.internal
  , :port 5432
  , :pool
    { :min 10
    , :max 50
    }
  , :ssl
    { :enabled true
    }
  }

, :redis
  { :cluster
    [ { :host redis-1.$region.acme.internal
      }
    , { :host redis-2.$region.acme.internal
      }
    ]
  }

, :logging
  { :level :warn
  , :format :json
  , :redact
    [ password
    , token
    , secret
    ]
  }

, :features
  { :new-checkout true
  , :ai-recs true
  , :beta-dash false
  }
}`,

  'API Response': `// E-commerce Order Response

{ :status :success
, :data
  { :order
    { :id ord_2xK9mN4pL8qR
    , :status :shipped
    , :customer
      { :email sarah@example.com
      , :name Sarah Johnson
      , :tier :gold
      }
    , :items
      [ { :sku WH-1000XM5
        , :name Sony Headphones
        , :quantity 1
        , :price 349.99
        }
      , { :sku USB-C-2M
        , :name USB-C Cable
        , :quantity 2
        , :price 19.99
        }
      ]
    , :summary
      { :subtotal 389.97
      , :discount 35.00
      , :tax 28.40
      , :total 383.37
      , :currency USD
      }
    , :payment
      { :method :credit_card
      , :brand :visa
      , :last4 4242
      , :status :captured
      }
    , :shipping
      { :carrier :fedex
      , :tracking 794644790138
      , :eta 2024-01-20
      }
    }
  }
}`,

  'Tailwind': `// Tailwind CSS Config

{ :content
  [ ./src/**/*.{js,tsx}
  , ./app/**/*.{js,tsx}
  ]
, :darkMode :class
, :theme
  { :extend
    { :colors
      { :brand
        { :50 #eff6ff
        , :500 #3b82f6
        , :900 #1e3a8a
        }
      , :success #10b981
      , :error #ef4444
      }
    , :fontFamily
      { :sans
        [ Inter
        , system-ui
        , sans-serif
        ]
      , :mono
        [ Fira Code
        , monospace
        ]
      }
    , :animation
      { :fade-in fade-in 0.5s ease-out
      }
    , :keyframes
      { :fade-in
        { :from
          { :opacity 0
          }
        , :to
          { :opacity 1
          }
        }
      }
    }
  }
, :plugins
  [ @tailwindcss/forms
  , @tailwindcss/typography
  ]
}`,
};

export default function App() {
  const [selected, setSelected] = useState(Object.keys(examples)[0]);
  return (
    <div className="min-h-screen bg-[#0d1117] text-[#e6edf3] p-4">
      <h1 className="text-xl font-bold mb-1 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">SYM Format Examples</h1>
      <p className="text-gray-500 mb-4 text-xs">Click tabs to browse real-world configurations</p>
      <div className="flex flex-wrap gap-1.5 mb-4">
        {Object.keys(examples).map(name => (
          <button key={name} onClick={() => setSelected(name)}
            className={`px-2.5 py-1 rounded text-xs font-medium transition-colors ${selected === name ? 'bg-green-600 text-white' : 'bg-[#21262d] text-gray-400 hover:bg-[#30363d]'}`}>
            {name}
          </button>
        ))}
      </div>
      <div className="bg-[#161b22] rounded-lg border border-[#30363d] overflow-hidden">
        <div className="px-3 py-2 border-b border-[#30363d] flex items-center gap-2">
          <div className="flex gap-1">
            <span className="w-2.5 h-2.5 rounded-full bg-red-500" />
            <span className="w-2.5 h-2.5 rounded-full bg-yellow-500" />
            <span className="w-2.5 h-2.5 rounded-full bg-green-500" />
          </div>
          <span className="ml-1 text-gray-500 text-xs">{selected.toLowerCase().replace(/ /g, '-').replace(/\./g, '')}.sym</span>
        </div>
        <div className="max-h-96 overflow-auto">
          <Highlight code={examples[selected]} />
        </div>
      </div>
      <div className="mt-4 text-[10px] text-gray-500 flex flex-wrap gap-x-2">
        <span className="text-gray-400">Legend:</span>
        <span><span style={{color: colors.comment}}>●</span> comments</span>
        <span><span style={{color: colors.variable}}>●</span> variables</span>
        <span><span style={{color: colors.key}}>●</span> keys</span>
        <span><span style={{color: colors.symbol}}>●</span> symbols</span>
        <span><span style={{color: colors.number}}>●</span> numbers</span>
        <span><span style={{color: colors.boolean}}>●</span> booleans</span>
        <span><span style={{color: colors.string}}>●</span> strings</span>
        <span><span style={{color: colors.bracket}}>●</span> brackets</span>
      </div>
    </div>
  );
}
